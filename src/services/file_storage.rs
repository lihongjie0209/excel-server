use std::fs;
use std::path::PathBuf;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::AppError;

#[derive(Clone)]
pub struct FileStorage {
    temp_dir: PathBuf,
    files: DashMap<String, StoredFile>,
    max_age_seconds: u64,
}

#[derive(Clone)]
struct StoredFile {
    file_id: String,
    filename: String,
    data: Vec<u8>,
    created_at: std::time::Instant,
}

/// 持久化的文件元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileMetadata {
    file_id: String,
    filename: String,
    created_timestamp: u64,
}

impl FileStorage {
    pub fn new(temp_dir: PathBuf, max_age_seconds: u64) -> Result<Self, AppError> {
        // 确保临时目录存在
        fs::create_dir_all(&temp_dir)?;
        
        let storage = Self {
            temp_dir: temp_dir.clone(),
            files: DashMap::new(),
            max_age_seconds,
        };
        
        // 从文件系统加载已存在的文件
        storage.load_from_filesystem()?;
        
        Ok(storage)
    }
    
    /// 存储文件并返回文件 ID
    pub async fn store(&self, filename: String, data: Vec<u8>) -> Result<String, AppError> {
        let file_id = Uuid::new_v4().to_string();
        
        // 写入文件到磁盘
        let file_path = self.get_file_path(&file_id);
        tokio::fs::write(&file_path, &data).await?;
        
        let stored = StoredFile {
            file_id: file_id.clone(),
            filename: filename.clone(),
            data,
            created_at: std::time::Instant::now(),
        };
        
        // 存储到内存
        self.files.insert(file_id.clone(), stored);
        
        // 持久化元数据
        self.save_metadata(&file_id, &filename)?;
        
        // 清理过期文件
        self.cleanup_expired();
        
        Ok(file_id)
    }
    
    /// 根据文件 ID 获取文件
    pub async fn retrieve(&self, file_id: &str) -> Result<(String, Vec<u8>), AppError> {
        if let Some(file) = self.files.get(file_id) {
            // 检查文件是否过期
            if file.created_at.elapsed().as_secs() > self.max_age_seconds {
                // 删除过期文件
                drop(file);
                self.delete(file_id).await?;
                return Err(AppError::NotFound(format!("文件已过期: {}", file_id)));
            }
            
            Ok((file.filename.clone(), file.data.clone()))
        } else {
            Err(AppError::NotFound(format!("文件不存在: {}", file_id)))
        }
    }
    
    /// 删除指定文件
    pub async fn delete(&self, file_id: &str) -> Result<(), AppError> {
        if self.files.remove(file_id).is_some() {
            // 从文件系统删除
            let file_path = self.get_file_path(file_id);
            let metadata_path = self.get_metadata_path(file_id);
            
            let _ = tokio::fs::remove_file(file_path).await;
            let _ = tokio::fs::remove_file(metadata_path).await;
            
            Ok(())
        } else {
            Err(AppError::NotFound(format!("文件不存在: {}", file_id)))
        }
    }
    
    /// 清理过期文件
    fn cleanup_expired(&self) {
        let expired_ids: Vec<String> = self.files
            .iter()
            .filter(|entry| entry.value().created_at.elapsed().as_secs() > self.max_age_seconds)
            .map(|entry| entry.key().clone())
            .collect();
        
        for file_id in expired_ids {
            let _ = tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    self.delete(&file_id).await
                })
            });
        }
    }
    
    /// 获取存储的文件数量
    pub async fn count(&self) -> usize {
        self.files.len()
    }
    
    /// 获取文件路径
    fn get_file_path(&self, file_id: &str) -> PathBuf {
        self.temp_dir.join(format!("{}.dat", file_id))
    }
    
    /// 获取元数据路径
    fn get_metadata_path(&self, file_id: &str) -> PathBuf {
        self.temp_dir.join(format!("{}.meta.json", file_id))
    }
    
    /// 保存文件元数据
    fn save_metadata(&self, file_id: &str, filename: &str) -> Result<(), AppError> {
        let metadata = FileMetadata {
            file_id: file_id.to_string(),
            filename: filename.to_string(),
            created_timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        
        let json = serde_json::to_string_pretty(&metadata)?;
        std::fs::write(self.get_metadata_path(file_id), json)?;
        
        Ok(())
    }
    
    /// 从文件系统加载已存在的文件
    fn load_from_filesystem(&self) -> Result<(), AppError> {
        if !self.temp_dir.exists() {
            return Ok(());
        }
        
        let entries = fs::read_dir(&self.temp_dir)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            // 只处理元数据文件
            if let Some(ext) = path.extension() {
                if ext == "json" {
                    if let Ok(json) = fs::read_to_string(&path) {
                        if let Ok(metadata) = serde_json::from_str::<FileMetadata>(&json) {
                            let file_path = self.get_file_path(&metadata.file_id);
                            
                            // 检查对应的数据文件是否存在
                            if file_path.exists() {
                                if let Ok(data) = fs::read(&file_path) {
                                    // 计算文件年龄
                                    let now = std::time::SystemTime::now()
                                        .duration_since(std::time::UNIX_EPOCH)
                                        .unwrap()
                                        .as_secs();
                                    let age = now.saturating_sub(metadata.created_timestamp);
                                    
                                    // 如果文件未过期，加载到内存
                                    if age <= self.max_age_seconds {
                                        let stored = StoredFile {
                                            file_id: metadata.file_id.clone(),
                                            filename: metadata.filename.clone(),
                                            data,
                                            created_at: std::time::Instant::now() - std::time::Duration::from_secs(age),
                                        };
                                        
                                        self.files.insert(metadata.file_id.clone(), stored);
                                    } else {
                                        // 删除过期文件
                                        let _ = fs::remove_file(&file_path);
                                        let _ = fs::remove_file(&path);
                                    }
                                }
                            } else {
                                // 数据文件不存在，删除元数据文件
                                let _ = fs::remove_file(&path);
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_store_and_retrieve() {
        let temp_dir = PathBuf::from("./temp_test");
        let storage = FileStorage::new(temp_dir.clone(), 3600).unwrap();
        
        let file_id = storage.store("test.xlsx".to_string(), vec![1, 2, 3]).await.unwrap();
        let (filename, data) = storage.retrieve(&file_id).await.unwrap();
        
        assert_eq!(filename, "test.xlsx");
        assert_eq!(data, vec![1, 2, 3]);
        
        // 清理
        let _ = fs::remove_dir_all(temp_dir);
    }
    
    #[tokio::test]
    async fn test_file_not_found() {
        let temp_dir = PathBuf::from("./temp_test2");
        let storage = FileStorage::new(temp_dir.clone(), 3600).unwrap();
        
        let result = storage.retrieve("non-existent-id").await;
        assert!(result.is_err());
        
        // 清理
        let _ = fs::remove_dir_all(temp_dir);
    }
}
