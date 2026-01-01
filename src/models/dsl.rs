use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

/// Excel DSL 顶层结构
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExcelDsl {
    /// 输出文件名
    #[schema(example = "report.xlsx")]
    pub filename: String,
    
    /// 文档元数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DocumentProperties>,
    
    /// 全局样式池
    #[serde(default)]
    pub styles: HashMap<String, Style>,
    
    /// 工作表集合
    pub sheets: Vec<Worksheet>,
}

/// 文档元数据
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DocumentProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
}

/// 样式定义
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Style {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<FontStyle>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<FillStyle>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<AlignStyle>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<BorderStyle>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect: Option<ProtectStyle>,
}

/// 字体样式
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FontStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
}

/// 填充样式
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FillStyle {
    pub color: String,
}

/// 对齐样式
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AlignStyle {
    /// 水平对齐: left, center, right
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<String>,
    
    /// 垂直对齐: top, vcenter, bottom
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_wrap: Option<bool>,
}

/// 边框样式
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BorderStyle {
    /// 四周边框线型 (1-13)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub around: Option<u8>,
}

/// 保护样式
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProtectStyle {
    pub locked: bool,
}

/// 工作表
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Worksheet {
    /// 工作表名称
    pub name: String,
    
    /// 单元格集合
    #[serde(default)]
    pub cells: Vec<Cell>,
    
    /// 合并单元格
    #[serde(default)]
    pub merges: Vec<RangeSpec>,
    
    /// 表格定义
    #[serde(default)]
    pub tables: Vec<Table>,
    
    /// 数据校验
    #[serde(default)]
    pub data_validations: Vec<DataValidation>,
    
    /// 条件格式
    #[serde(default)]
    pub conditional_formats: Vec<ConditionalFormat>,
    
    /// 迷你图
    #[serde(default)]
    pub sparklines: Vec<Sparkline>,
}

/// 单元格
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Cell {
    /// 行索引 (0-based)
    pub r: u32,
    
    /// 列索引 (0-based)
    pub c: u16,
    
    /// 数据类型
    #[serde(rename = "type")]
    pub cell_type: CellType,
    
    /// 单元格值
    pub value: CellValue,
    
    /// 样式引用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

/// 单元格类型
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum CellType {
    String,
    Number,
    Datetime,
    Formula,
}

/// 单元格值（支持多种类型）
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(untagged)]
pub enum CellValue {
    String(String),
    Number(f64),
    Bool(bool),
}

/// 范围描述符（支持 A1 引用和坐标系）
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(untagged)]
pub enum RangeSpec {
    /// A1 引用格式 (如 "A1:C10")
    A1(String),
    
    /// 坐标系格式
    Coords(RangeCoords),
}

/// 坐标系范围
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RangeCoords {
    pub r1: u32,
    pub c1: u16,
    pub r2: u32,
    pub c2: u16,
}

/// 表格定义
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Table {
    pub range: RangeSpec,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    
    pub columns: Vec<TableColumn>,
}

/// 表格列
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TableColumn {
    pub header: String,
}

/// 数据校验
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DataValidation {
    pub range: RangeSpec,
    
    /// 校验类型: list, integer, decimal, date, time
    #[serde(rename = "type")]
    pub validation_type: String,
    
    /// 允许的值
    pub value: serde_json::Value,
}

/// 条件格式
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ConditionalFormat {
    pub range: RangeSpec,
    
    /// 条件类型: cell, data_bar, color_scale
    #[serde(rename = "type")]
    pub format_type: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criteria: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

/// 迷你图
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Sparkline {
    /// 插入位置
    pub location: LocationSpec,
    
    /// 数据源范围 (需带表名引用，如 "Sheet1!A1:D1")
    pub range: String,
}

/// 位置描述符
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(untagged)]
pub enum LocationSpec {
    /// A1 引用 (如 "E1")
    A1(String),
    
    /// 坐标系
    Coords(LocationCoords),
}

/// 坐标系位置
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LocationCoords {
    pub r: u32,
    pub c: u16,
}
