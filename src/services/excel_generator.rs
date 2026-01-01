use rust_xlsxwriter::{
    Color, ConditionalFormatCell, ConditionalFormatCellRule, ConditionalFormatDataBar,
    DocProperties, Format, FormatAlign, FormatBorder, Workbook,
    Worksheet as XlsxWorksheet, Table as XlsxTable, TableColumn as XlsxTableColumn,
    DataValidation as XlsxDataValidation,
};
use std::collections::HashMap;

use crate::errors::AppError;
use crate::models::*;

pub struct ExcelGenerator {
    styles_cache: HashMap<String, Format>,
}

impl ExcelGenerator {
    pub fn new() -> Self {
        Self {
            styles_cache: HashMap::new(),
        }
    }
    
    /// 根据 DSL 生成 Excel 文件并返回字节数组
    pub fn generate(&mut self, dsl: &ExcelDsl) -> Result<Vec<u8>, AppError> {
        let mut workbook = Workbook::new();
        
        // 设置文档属性
        if let Some(props) = &dsl.properties {
            let mut doc_props = DocProperties::new();
            if let Some(title) = &props.title {
                doc_props = doc_props.set_title(title);
            }
            if let Some(author) = &props.author {
                doc_props = doc_props.set_author(author);
            }
            if let Some(company) = &props.company {
                doc_props = doc_props.set_company(company);
            }
            workbook.set_properties(&doc_props);
        }
        
        // 预处理样式
        self.build_styles(&dsl.styles)?;
        
        // 生成所有工作表
        for sheet_def in &dsl.sheets {
            self.build_worksheet(&mut workbook, sheet_def)?;
        }
        
        // 保存到内存缓冲区
        let buffer = workbook.save_to_buffer()?;
        Ok(buffer)
    }
    
    /// 构建样式缓存
    fn build_styles(&mut self, styles: &HashMap<String, Style>) -> Result<(), AppError> {
        self.styles_cache.clear();
        
        for (style_id, style) in styles {
            let format = self.create_format(style)?;
            self.styles_cache.insert(style_id.clone(), format);
        }
        
        Ok(())
    }
    
    /// 创建格式对象
    fn create_format(&self, style: &Style) -> Result<Format, AppError> {
        let mut format = Format::new();
        
        // 字体样式
        if let Some(font) = &style.font {
            if let Some(bold) = font.bold {
                if bold {
                    format = format.set_bold();
                }
            }
            if let Some(italic) = font.italic {
                if italic {
                    format = format.set_italic();
                }
            }
            if let Some(color) = &font.color {
                if let Some(color_obj) = parse_color(color) {
                    format = format.set_font_color(color_obj);
                }
            }
            if let Some(size) = font.size {
                format = format.set_font_size(size);
            }
        }
        
        // 填充样式
        if let Some(fill) = &style.fill {
            if let Some(color) = parse_color(&fill.color) {
                format = format.set_background_color(color);
            }
        }
        
        // 对齐样式
        if let Some(align) = &style.align {
            if let Some(h) = &align.h {
                format = match h.as_str() {
                    "left" => format.set_align(FormatAlign::Left),
                    "center" => format.set_align(FormatAlign::Center),
                    "right" => format.set_align(FormatAlign::Right),
                    _ => format,
                };
            }
            if let Some(v) = &align.v {
                format = match v.as_str() {
                    "top" => format.set_align(FormatAlign::Top),
                    "vcenter" => format.set_align(FormatAlign::VerticalCenter),
                    "bottom" => format.set_align(FormatAlign::Bottom),
                    _ => format,
                };
            }
            if let Some(text_wrap) = align.text_wrap {
                if text_wrap {
                    format = format.set_text_wrap();
                }
            }
        }
        
        // 边框样式
        if let Some(border) = &style.border {
            if let Some(around) = border.around {
                let border_type = match around {
                    1 => FormatBorder::Thin,
                    2 => FormatBorder::Medium,
                    3 => FormatBorder::Dashed,
                    4 => FormatBorder::Dotted,
                    5 => FormatBorder::Thick,
                    6 => FormatBorder::Double,
                    _ => FormatBorder::Thin,
                };
                format = format.set_border(border_type);
            }
        }
        
        // 保护样式
        if let Some(protect) = &style.protect {
            if protect.locked {
                format = format.set_locked();
            } else {
                format = format.set_unlocked();
            }
        }
        
        Ok(format)
    }
    
    /// 构建工作表
    fn build_worksheet(&self, workbook: &mut Workbook, sheet: &Worksheet) -> Result<(), AppError> {
        let mut worksheet = workbook.add_worksheet();
        worksheet.set_name(&sheet.name)?;
        
        // 写入单元格
        for cell in &sheet.cells {
            self.write_cell(&mut worksheet, cell)?;
        }
        
        // 合并单元格
        for merge in &sheet.merges {
            self.apply_merge(&mut worksheet, merge)?;
        }
        
        // 添加表格
        for table in &sheet.tables {
            self.add_table(&mut worksheet, table)?;
        }
        
        // 数据校验
        for validation in &sheet.data_validations {
            self.add_data_validation(&mut worksheet, validation)?;
        }
        
        // 条件格式
        for cond_format in &sheet.conditional_formats {
            self.add_conditional_format(&mut worksheet, cond_format)?;
        }
        
        Ok(())
    }
    
    /// 写入单元格
    fn write_cell(&self, worksheet: &mut XlsxWorksheet, cell: &Cell) -> Result<(), AppError> {
        let format = cell.style.as_ref()
            .and_then(|style_id| self.styles_cache.get(style_id));
        
        match &cell.cell_type {
            CellType::String => {
                if let CellValue::String(s) = &cell.value {
                    if let Some(fmt) = format {
                        worksheet.write_string_with_format(cell.r, cell.c, s, fmt)?;
                    } else {
                        worksheet.write_string(cell.r, cell.c, s)?;
                    }
                }
            }
            CellType::Number => {
                if let CellValue::Number(n) = &cell.value {
                    if let Some(fmt) = format {
                        worksheet.write_number_with_format(cell.r, cell.c, *n, fmt)?;
                    } else {
                        worksheet.write_number(cell.r, cell.c, *n)?;
                    }
                }
            }
            CellType::Formula => {
                if let CellValue::String(f) = &cell.value {
                    if let Some(fmt) = format {
                        worksheet.write_formula_with_format(cell.r, cell.c, f.as_str(), fmt)?;
                    } else {
                        worksheet.write_formula(cell.r, cell.c, f.as_str())?;
                    }
                }
            }
            CellType::Datetime => {
                // 日期时间处理需要额外的格式设置
                if let CellValue::String(s) = &cell.value {
                    // 这里可以使用 chrono 解析日期并写入
                    // 简化处理，直接作为字符串
                    if let Some(fmt) = format {
                        worksheet.write_string_with_format(cell.r, cell.c, s, fmt)?;
                    } else {
                        worksheet.write_string(cell.r, cell.c, s)?;
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// 应用合并单元格
    fn apply_merge(&self, worksheet: &mut XlsxWorksheet, merge: &RangeSpec) -> Result<(), AppError> {
        let (r1, c1, r2, c2) = parse_range(merge)?;
        worksheet.merge_range(r1, c1, r2, c2, "", &Format::new())?;
        Ok(())
    }
    
    /// 添加表格
    fn add_table(&self, worksheet: &mut XlsxWorksheet, table: &Table) -> Result<(), AppError> {
        let (r1, c1, r2, c2) = parse_range(&table.range)?;
        
        let mut table_obj = XlsxTable::new();
        
        // 设置列（使用链式调用，因为add_column返回TableColumn而不是Table）
        let mut columns = Vec::new();
        for col_def in &table.columns {
            let column = XlsxTableColumn::new().set_header(&col_def.header);
            columns.push(column);
        }
        
        // 逐个设置列
        for column in columns {
            table_obj = table_obj.set_columns(&[column]);
        }
        
        // 设置表格样式
        if let Some(_style_name) = &table.style {
            // rust_xlsxwriter 支持预定义的表格样式
            // 这里简化处理
        }
        
        worksheet.add_table(r1, c1, r2, c2, &table_obj)?;
        Ok(())
    }
    
    /// 添加数据校验
    fn add_data_validation(&self, worksheet: &mut XlsxWorksheet, validation: &DataValidation) -> Result<(), AppError> {
        let (r1, c1, r2, c2) = parse_range(&validation.range)?;
        
        match validation.validation_type.as_str() {
            "list" => {
                if let Some(list) = validation.value.as_array() {
                    let values: Vec<String> = list.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect();
                    
                    let mut data_val = XlsxDataValidation::new();
                    data_val = data_val.allow_list_strings(&values)?;
                    
                    worksheet.add_data_validation(r1, c1, r2, c2, &data_val)?;
                }
            }
            _ => {
                // 其他类型的校验可以继续扩展
            }
        }
        
        Ok(())
    }
    
    /// 添加条件格式
    fn add_conditional_format(&self, worksheet: &mut XlsxWorksheet, cond_format: &ConditionalFormat) -> Result<(), AppError> {
        let (r1, c1, r2, c2) = parse_range(&cond_format.range)?;
        
        match cond_format.format_type.as_str() {
            "cell" => {
                if let (Some(criteria), Some(value)) = (&cond_format.criteria, &cond_format.value) {
                    let mut cond_fmt = ConditionalFormatCell::new();
                    
                    // 根据条件类型设置
                    if criteria == ">" {
                        if let Some(num) = value.as_f64() {
                            cond_fmt = cond_fmt.set_rule(ConditionalFormatCellRule::GreaterThan(num));
                        }
                    }
                    
                    // 应用样式
                    if let Some(style_id) = &cond_format.style {
                        if let Some(format) = self.styles_cache.get(style_id) {
                            cond_fmt = cond_fmt.set_format(format);
                        }
                    }
                    
                    worksheet.add_conditional_format(r1, c1, r2, c2, &cond_fmt)?;
                }
            }
            "data_bar" => {
                let data_bar = ConditionalFormatDataBar::new();
                worksheet.add_conditional_format(r1, c1, r2, c2, &data_bar)?;
            }
            _ => {
                // 其他类型可以继续扩展
            }
        }
        
        Ok(())
    }
}

/// 解析颜色字符串为 Color 对象
fn parse_color(color_str: &str) -> Option<Color> {
    // 移除 # 符号
    let hex = color_str.trim_start_matches('#');
    
    if hex.len() == 6 {
        if let Ok(rgb) = u32::from_str_radix(hex, 16) {
            return Some(Color::RGB(rgb));
        }
    }
    
    None
}

/// 解析范围描述符为坐标
fn parse_range(range: &RangeSpec) -> Result<(u32, u16, u32, u16), AppError> {
    match range {
        RangeSpec::A1(a1) => {
            // 解析 A1 格式，例如 "A1:C10"
            parse_a1_range(a1)
        }
        RangeSpec::Coords(coords) => {
            Ok((coords.r1, coords.c1, coords.r2, coords.c2))
        }
    }
}

/// 解析 A1 格式的范围
fn parse_a1_range(a1: &str) -> Result<(u32, u16, u32, u16), AppError> {
    if let Some((start, end)) = a1.split_once(':') {
        let (r1, c1) = parse_a1_cell(start)?;
        let (r2, c2) = parse_a1_cell(end)?;
        Ok((r1, c1, r2, c2))
    } else {
        let (r, c) = parse_a1_cell(a1)?;
        Ok((r, c, r, c))
    }
}

/// 解析单个 A1 格式的单元格引用
fn parse_a1_cell(cell: &str) -> Result<(u32, u16), AppError> {
    let cell = cell.trim();
    let mut col_str = String::new();
    let mut row_str = String::new();
    
    for ch in cell.chars() {
        if ch.is_alphabetic() {
            col_str.push(ch.to_ascii_uppercase());
        } else if ch.is_numeric() {
            row_str.push(ch);
        }
    }
    
    let col = a1_to_col(&col_str)?;
    let row = row_str.parse::<u32>()
        .map_err(|_| AppError::ValidationError(format!("无效的行号: {}", row_str)))?
        .saturating_sub(1); // Excel 行号从 1 开始，转为 0-based
    
    Ok((row, col))
}

/// 将 A1 列字母转换为数字索引
fn a1_to_col(col: &str) -> Result<u16, AppError> {
    let mut result = 0u16;
    for ch in col.chars() {
        if !ch.is_ascii_uppercase() {
            return Err(AppError::ValidationError(format!("无效的列名: {}", col)));
        }
        result = result * 26 + (ch as u16 - 'A' as u16 + 1);
    }
    Ok(result.saturating_sub(1)) // 转为 0-based
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_parse_a1_cell() {
        assert_eq!(parse_a1_cell("A1").unwrap(), (0, 0));
        assert_eq!(parse_a1_cell("B2").unwrap(), (1, 1));
        assert_eq!(parse_a1_cell("Z26").unwrap(), (25, 25));
        assert_eq!(parse_a1_cell("AA1").unwrap(), (0, 26));
        assert_eq!(parse_a1_cell("AB10").unwrap(), (9, 27));
    }
    
    #[test]
    fn test_a1_to_col() {
        assert_eq!(a1_to_col("A").unwrap(), 0);
        assert_eq!(a1_to_col("Z").unwrap(), 25);
        assert_eq!(a1_to_col("AA").unwrap(), 26);
        assert_eq!(a1_to_col("AB").unwrap(), 27);
        assert_eq!(a1_to_col("AZ").unwrap(), 51);
    }
    
    #[test]
    fn test_a1_to_col_invalid() {
        assert!(a1_to_col("1").is_err());
        assert!(a1_to_col("a").is_err());
        assert!(a1_to_col("A1").is_err());
    }
    
    #[test]
    fn test_parse_a1_range() {
        assert_eq!(parse_a1_range("A1:B2").unwrap(), (0, 0, 1, 1));
        assert_eq!(parse_a1_range("C3:E5").unwrap(), (2, 2, 4, 4));
        assert_eq!(parse_a1_range("A1").unwrap(), (0, 0, 0, 0));
    }
    
    #[test]
    fn test_parse_range_a1() {
        let range = RangeSpec::A1("A1:C10".to_string());
        let result = parse_range(&range).unwrap();
        assert_eq!(result, (0, 0, 9, 2));
    }
    
    #[test]
    fn test_parse_range_coords() {
        let range = RangeSpec::Coords(RangeCoords {
            r1: 0,
            c1: 0,
            r2: 9,
            c2: 2,
        });
        let result = parse_range(&range).unwrap();
        assert_eq!(result, (0, 0, 9, 2));
    }
    
    #[test]
    fn test_parse_color() {
        assert!(parse_color("#FF0000").is_some());
        assert!(parse_color("#00FF00").is_some());
        assert!(parse_color("#0000FF").is_some());
        assert!(parse_color("#FFFFFF").is_some());
        assert!(parse_color("#000000").is_some());
        
        // Test without #
        assert!(parse_color("FF0000").is_some());
        
        // Invalid colors
        assert!(parse_color("#FFF").is_none());
        assert!(parse_color("#GGGGGG").is_none());
        assert!(parse_color("").is_none());
    }
    
    #[test]
    fn test_create_format_with_font() {
        let generator = ExcelGenerator::new();
        
        let style = Style {
            font: Some(FontStyle {
                bold: Some(true),
                italic: Some(true),
                color: Some("#FF0000".to_string()),
                size: Some(14.0),
            }),
            fill: None,
            align: None,
            border: None,
            protect: None,
        };
        
        let format = generator.create_format(&style);
        assert!(format.is_ok());
    }
    
    #[test]
    fn test_create_format_with_fill() {
        let generator = ExcelGenerator::new();
        
        let style = Style {
            font: None,
            fill: Some(FillStyle {
                color: "#FFFF00".to_string(),
            }),
            align: None,
            border: None,
            protect: None,
        };
        
        let format = generator.create_format(&style);
        assert!(format.is_ok());
    }
    
    #[test]
    fn test_create_format_with_align() {
        let generator = ExcelGenerator::new();
        
        let style = Style {
            font: None,
            fill: None,
            align: Some(AlignStyle {
                h: Some("center".to_string()),
                v: Some("vcenter".to_string()),
                text_wrap: Some(true),
            }),
            border: None,
            protect: None,
        };
        
        let format = generator.create_format(&style);
        assert!(format.is_ok());
    }
    
    #[test]
    fn test_create_format_with_border() {
        let generator = ExcelGenerator::new();
        
        let style = Style {
            font: None,
            fill: None,
            align: None,
            border: Some(BorderStyle {
                around: Some(1),
            }),
            protect: None,
        };
        
        let format = generator.create_format(&style);
        assert!(format.is_ok());
    }
    
    #[test]
    fn test_create_format_with_protect() {
        let generator = ExcelGenerator::new();
        
        let style = Style {
            font: None,
            fill: None,
            align: None,
            border: None,
            protect: Some(ProtectStyle {
                locked: true,
            }),
        };
        
        let format = generator.create_format(&style);
        assert!(format.is_ok());
    }
    
    #[test]
    fn test_build_styles() {
        let mut generator = ExcelGenerator::new();
        
        let mut styles = HashMap::new();
        styles.insert("header".to_string(), Style {
            font: Some(FontStyle {
                bold: Some(true),
                italic: None,
                color: Some("#FFFFFF".to_string()),
                size: None,
            }),
            fill: Some(FillStyle {
                color: "#4472C4".to_string(),
            }),
            align: None,
            border: None,
            protect: None,
        });
        
        let result = generator.build_styles(&styles);
        assert!(result.is_ok());
        assert_eq!(generator.styles_cache.len(), 1);
        assert!(generator.styles_cache.contains_key("header"));
    }
    
    #[test]
    fn test_generate_simple_excel() {
        let mut generator = ExcelGenerator::new();
        
        let dsl = ExcelDsl {
            filename: "test.xlsx".to_string(),
            properties: Some(DocumentProperties {
                title: Some("Test".to_string()),
                author: Some("Test Author".to_string()),
                company: Some("Test Company".to_string()),
            }),
            styles: HashMap::new(),
            sheets: vec![
                Worksheet {
                    name: "Sheet1".to_string(),
                    cells: vec![
                        Cell {
                            r: 0,
                            c: 0,
                            cell_type: CellType::String,
                            value: CellValue::String("Hello".to_string()),
                            style: None,
                        },
                        Cell {
                            r: 0,
                            c: 1,
                            cell_type: CellType::Number,
                            value: CellValue::Number(42.0),
                            style: None,
                        },
                    ],
                    merges: vec![],
                    tables: vec![],
                    data_validations: vec![],
                    conditional_formats: vec![],
                    sparklines: vec![],
                }
            ],
        };
        
        let result = generator.generate(&dsl);
        assert!(result.is_ok());
        
        let buffer = result.unwrap();
        assert!(!buffer.is_empty());
        
        // Check for Excel magic bytes (ZIP file signature)
        assert_eq!(&buffer[0..2], &[0x50, 0x4B]);
    }
    
    #[test]
    fn test_generate_excel_with_formula() {
        let mut generator = ExcelGenerator::new();
        
        let dsl = ExcelDsl {
            filename: "test_formula.xlsx".to_string(),
            properties: None,
            styles: HashMap::new(),
            sheets: vec![
                Worksheet {
                    name: "Sheet1".to_string(),
                    cells: vec![
                        Cell {
                            r: 0,
                            c: 0,
                            cell_type: CellType::Number,
                            value: CellValue::Number(10.0),
                            style: None,
                        },
                        Cell {
                            r: 1,
                            c: 0,
                            cell_type: CellType::Number,
                            value: CellValue::Number(20.0),
                            style: None,
                        },
                        Cell {
                            r: 2,
                            c: 0,
                            cell_type: CellType::Formula,
                            value: CellValue::String("=A1+A2".to_string()),
                            style: None,
                        },
                    ],
                    merges: vec![],
                    tables: vec![],
                    data_validations: vec![],
                    conditional_formats: vec![],
                    sparklines: vec![],
                }
            ],
        };
        
        let result = generator.generate(&dsl);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_generate_excel_with_merge() {
        let mut generator = ExcelGenerator::new();
        
        let dsl = ExcelDsl {
            filename: "test_merge.xlsx".to_string(),
            properties: None,
            styles: HashMap::new(),
            sheets: vec![
                Worksheet {
                    name: "Sheet1".to_string(),
                    cells: vec![],
                    merges: vec![
                        RangeSpec::A1("A1:B2".to_string()),
                        RangeSpec::Coords(RangeCoords {
                            r1: 3,
                            c1: 0,
                            r2: 4,
                            c2: 1,
                        }),
                    ],
                    tables: vec![],
                    data_validations: vec![],
                    conditional_formats: vec![],
                    sparklines: vec![],
                }
            ],
        };
        
        let result = generator.generate(&dsl);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_generate_excel_with_styles() {
        let mut generator = ExcelGenerator::new();
        
        let mut styles = HashMap::new();
        styles.insert("header".to_string(), Style {
            font: Some(FontStyle {
                bold: Some(true),
                italic: None,
                color: Some("#FFFFFF".to_string()),
                size: Some(12.0),
            }),
            fill: Some(FillStyle {
                color: "#4472C4".to_string(),
            }),
            align: Some(AlignStyle {
                h: Some("center".to_string()),
                v: Some("vcenter".to_string()),
                text_wrap: None,
            }),
            border: Some(BorderStyle {
                around: Some(1),
            }),
            protect: None,
        });
        
        let dsl = ExcelDsl {
            filename: "test_styles.xlsx".to_string(),
            properties: None,
            styles,
            sheets: vec![
                Worksheet {
                    name: "Sheet1".to_string(),
                    cells: vec![
                        Cell {
                            r: 0,
                            c: 0,
                            cell_type: CellType::String,
                            value: CellValue::String("Header".to_string()),
                            style: Some("header".to_string()),
                        },
                    ],
                    merges: vec![],
                    tables: vec![],
                    data_validations: vec![],
                    conditional_formats: vec![],
                    sparklines: vec![],
                }
            ],
        };
        
        let result = generator.generate(&dsl);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_generate_excel_with_data_validation() {
        let mut generator = ExcelGenerator::new();
        
        let dsl = ExcelDsl {
            filename: "test_validation.xlsx".to_string(),
            properties: None,
            styles: HashMap::new(),
            sheets: vec![
                Worksheet {
                    name: "Sheet1".to_string(),
                    cells: vec![],
                    merges: vec![],
                    tables: vec![],
                    data_validations: vec![
                        DataValidation {
                            range: RangeSpec::A1("A1:A10".to_string()),
                            validation_type: "list".to_string(),
                            value: json!(["Option1", "Option2", "Option3"]),
                        },
                    ],
                    conditional_formats: vec![],
                    sparklines: vec![],
                }
            ],
        };
        
        let result = generator.generate(&dsl);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_generate_excel_with_conditional_format() {
        let mut generator = ExcelGenerator::new();
        
        let mut styles = HashMap::new();
        styles.insert("highlight".to_string(), Style {
            font: Some(FontStyle {
                bold: Some(true),
                italic: None,
                color: Some("#FF0000".to_string()),
                size: None,
            }),
            fill: None,
            align: None,
            border: None,
            protect: None,
        });
        
        let dsl = ExcelDsl {
            filename: "test_conditional.xlsx".to_string(),
            properties: None,
            styles,
            sheets: vec![
                Worksheet {
                    name: "Sheet1".to_string(),
                    cells: vec![],
                    merges: vec![],
                    tables: vec![],
                    data_validations: vec![],
                    conditional_formats: vec![
                        ConditionalFormat {
                            range: RangeSpec::A1("A1:A10".to_string()),
                            format_type: "cell".to_string(),
                            criteria: Some(">".to_string()),
                            value: Some(json!(50)),
                            style: Some("highlight".to_string()),
                        },
                        ConditionalFormat {
                            range: RangeSpec::Coords(RangeCoords {
                                r1: 0,
                                c1: 1,
                                r2: 9,
                                c2: 1,
                            }),
                            format_type: "data_bar".to_string(),
                            criteria: None,
                            value: None,
                            style: None,
                        },
                    ],
                    sparklines: vec![],
                }
            ],
        };
        
        let result = generator.generate(&dsl);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_all_cell_types() {
        let mut generator = ExcelGenerator::new();
        
        let dsl = ExcelDsl {
            filename: "test_cell_types.xlsx".to_string(),
            properties: None,
            styles: HashMap::new(),
            sheets: vec![
                Worksheet {
                    name: "Sheet1".to_string(),
                    cells: vec![
                        Cell {
                            r: 0,
                            c: 0,
                            cell_type: CellType::String,
                            value: CellValue::String("Text".to_string()),
                            style: None,
                        },
                        Cell {
                            r: 1,
                            c: 0,
                            cell_type: CellType::Number,
                            value: CellValue::Number(123.45),
                            style: None,
                        },
                        Cell {
                            r: 2,
                            c: 0,
                            cell_type: CellType::Formula,
                            value: CellValue::String("=SUM(A1:A2)".to_string()),
                            style: None,
                        },
                        Cell {
                            r: 3,
                            c: 0,
                            cell_type: CellType::Datetime,
                            value: CellValue::String("2024-01-01".to_string()),
                            style: None,
                        },
                    ],
                    merges: vec![],
                    tables: vec![],
                    data_validations: vec![],
                    conditional_formats: vec![],
                    sparklines: vec![],
                }
            ],
        };
        
        let result = generator.generate(&dsl);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_all_align_types() {
        let generator = ExcelGenerator::new();
        
        let style_left = Style {
            font: None,
            fill: None,
            align: Some(AlignStyle {
                h: Some("left".to_string()),
                v: Some("top".to_string()),
                text_wrap: None,
            }),
            border: None,
            protect: None,
        };
        assert!(generator.create_format(&style_left).is_ok());
        
        let style_center = Style {
            font: None,
            fill: None,
            align: Some(AlignStyle {
                h: Some("center".to_string()),
                v: Some("vcenter".to_string()),
                text_wrap: None,
            }),
            border: None,
            protect: None,
        };
        assert!(generator.create_format(&style_center).is_ok());
        
        let style_right = Style {
            font: None,
            fill: None,
            align: Some(AlignStyle {
                h: Some("right".to_string()),
                v: Some("bottom".to_string()),
                text_wrap: None,
            }),
            border: None,
            protect: None,
        };
        assert!(generator.create_format(&style_right).is_ok());
    }
    
    #[test]
    fn test_all_border_types() {
        let generator = ExcelGenerator::new();
        
        for border_type in 1..=6 {
            let style = Style {
                font: None,
                fill: None,
                align: None,
                border: Some(BorderStyle {
                    around: Some(border_type),
                }),
                protect: None,
            };
            assert!(generator.create_format(&style).is_ok());
        }
    }
    
    #[test]
    fn test_parse_a1_cell_edge_cases() {
        // Test with whitespace
        assert_eq!(parse_a1_cell("  A1  ").unwrap(), (0, 0));
        
        // Test uppercase and lowercase
        assert_eq!(parse_a1_cell("a1").unwrap(), (0, 0));
        assert_eq!(parse_a1_cell("B2").unwrap(), (1, 1));
    }
    
    #[test]
    fn test_parse_a1_cell_errors() {
        // Empty string
        assert!(parse_a1_cell("").is_err());
        
        // Only letters - this should work but return error for missing row
        // Note: parse_a1_cell("ABC") will try to parse and fail on empty row_str
        let result = parse_a1_cell("ABC");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_parse_color_edge_cases() {
        // Test various valid colors
        assert!(parse_color("#123456").is_some());
        assert!(parse_color("#ABCDEF").is_some());
        assert!(parse_color("#abcdef").is_some());
        
        // Invalid lengths
        assert!(parse_color("#FF").is_none());
        assert!(parse_color("#FFFFFFF").is_none());
        
        // Invalid characters
        assert!(parse_color("#GGGGGG").is_none());
        assert!(parse_color("#12345G").is_none());
    }
    
    #[test]
    fn test_empty_styles() {
        let generator = ExcelGenerator::new();
        
        let style = Style {
            font: None,
            fill: None,
            align: None,
            border: None,
            protect: None,
        };
        
        assert!(generator.create_format(&style).is_ok());
    }
    
    #[test]
    fn test_generate_with_no_properties() {
        let mut generator = ExcelGenerator::new();
        
        let dsl = ExcelDsl {
            filename: "test_no_props.xlsx".to_string(),
            properties: None,
            styles: HashMap::new(),
            sheets: vec![
                Worksheet {
                    name: "Sheet1".to_string(),
                    cells: vec![],
                    merges: vec![],
                    tables: vec![],
                    data_validations: vec![],
                    conditional_formats: vec![],
                    sparklines: vec![],
                }
            ],
        };
        
        let result = generator.generate(&dsl);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_generate_multiple_sheets() {
        let mut generator = ExcelGenerator::new();
        
        let dsl = ExcelDsl {
            filename: "test_multi_sheets.xlsx".to_string(),
            properties: None,
            styles: HashMap::new(),
            sheets: vec![
                Worksheet {
                    name: "Sheet1".to_string(),
                    cells: vec![
                        Cell {
                            r: 0,
                            c: 0,
                            cell_type: CellType::String,
                            value: CellValue::String("Sheet 1".to_string()),
                            style: None,
                        },
                    ],
                    merges: vec![],
                    tables: vec![],
                    data_validations: vec![],
                    conditional_formats: vec![],
                    sparklines: vec![],
                },
                Worksheet {
                    name: "Sheet2".to_string(),
                    cells: vec![
                        Cell {
                            r: 0,
                            c: 0,
                            cell_type: CellType::String,
                            value: CellValue::String("Sheet 2".to_string()),
                            style: None,
                        },
                    ],
                    merges: vec![],
                    tables: vec![],
                    data_validations: vec![],
                    conditional_formats: vec![],
                    sparklines: vec![],
                },
            ],
        };
        
        let result = generator.generate(&dsl);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_font_style_with_only_bold() {
        let generator = ExcelGenerator::new();
        
        let style = Style {
            font: Some(FontStyle {
                bold: Some(true),
                italic: None,
                color: None,
                size: None,
            }),
            fill: None,
            align: None,
            border: None,
            protect: None,
        };
        
        assert!(generator.create_format(&style).is_ok());
    }
    
    #[test]
    fn test_font_style_with_only_italic() {
        let generator = ExcelGenerator::new();
        
        let style = Style {
            font: Some(FontStyle {
                bold: None,
                italic: Some(true),
                color: None,
                size: None,
            }),
            fill: None,
            align: None,
            border: None,
            protect: None,
        };
        
        assert!(generator.create_format(&style).is_ok());
    }
    
    #[test]
    fn test_font_style_false_values() {
        let generator = ExcelGenerator::new();
        
        let style = Style {
            font: Some(FontStyle {
                bold: Some(false),
                italic: Some(false),
                color: None,
                size: None,
            }),
            fill: None,
            align: None,
            border: None,
            protect: None,
        };
        
        assert!(generator.create_format(&style).is_ok());
    }
    
    #[test]
    fn test_align_with_unknown_values() {
        let generator = ExcelGenerator::new();
        
        let style = Style {
            font: None,
            fill: None,
            align: Some(AlignStyle {
                h: Some("unknown".to_string()),
                v: Some("unknown".to_string()),
                text_wrap: Some(false),
            }),
            border: None,
            protect: None,
        };
        
        assert!(generator.create_format(&style).is_ok());
    }
    
    #[test]
    fn test_border_with_unknown_type() {
        let generator = ExcelGenerator::new();
        
        let style = Style {
            font: None,
            fill: None,
            align: None,
            border: Some(BorderStyle {
                around: Some(99),  // Unknown border type
            }),
            protect: None,
        };
        
        assert!(generator.create_format(&style).is_ok());
    }
    
    #[test]
    fn test_protect_unlocked() {
        let generator = ExcelGenerator::new();
        
        let style = Style {
            font: None,
            fill: None,
            align: None,
            border: None,
            protect: Some(ProtectStyle {
                locked: false,
            }),
        };
        
        assert!(generator.create_format(&style).is_ok());
    }
    
    #[test]
    fn test_cell_with_bool_value() {
        let mut generator = ExcelGenerator::new();
        
        let dsl = ExcelDsl {
            filename: "test_bool.xlsx".to_string(),
            properties: None,
            styles: HashMap::new(),
            sheets: vec![
                Worksheet {
                    name: "Sheet1".to_string(),
                    cells: vec![
                        Cell {
                            r: 0,
                            c: 0,
                            cell_type: CellType::String,
                            value: CellValue::Bool(true),
                            style: None,
                        },
                    ],
                    merges: vec![],
                    tables: vec![],
                    data_validations: vec![],
                    conditional_formats: vec![],
                    sparklines: vec![],
                }
            ],
        };
        
        let result = generator.generate(&dsl);
        // Should still work even though type mismatch
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_data_validation_with_non_array() {
        let mut generator = ExcelGenerator::new();
        
        let dsl = ExcelDsl {
            filename: "test_validation_error.xlsx".to_string(),
            properties: None,
            styles: HashMap::new(),
            sheets: vec![
                Worksheet {
                    name: "Sheet1".to_string(),
                    cells: vec![],
                    merges: vec![],
                    tables: vec![],
                    data_validations: vec![
                        DataValidation {
                            range: RangeSpec::A1("A1:A10".to_string()),
                            validation_type: "list".to_string(),
                            value: json!("not an array"),
                        },
                    ],
                    conditional_formats: vec![],
                    sparklines: vec![],
                }
            ],
        };
        
        let result = generator.generate(&dsl);
        // Should succeed but not add validation
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_data_validation_with_unknown_type() {
        let mut generator = ExcelGenerator::new();
        
        let dsl = ExcelDsl {
            filename: "test_validation_unknown.xlsx".to_string(),
            properties: None,
            styles: HashMap::new(),
            sheets: vec![
                Worksheet {
                    name: "Sheet1".to_string(),
                    cells: vec![],
                    merges: vec![],
                    tables: vec![],
                    data_validations: vec![
                        DataValidation {
                            range: RangeSpec::A1("A1:A10".to_string()),
                            validation_type: "unknown".to_string(),
                            value: json!(["test"]),
                        },
                    ],
                    conditional_formats: vec![],
                    sparklines: vec![],
                }
            ],
        };
        
        let result = generator.generate(&dsl);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_conditional_format_with_unknown_type() {
        let mut generator = ExcelGenerator::new();
        
        let dsl = ExcelDsl {
            filename: "test_cf_unknown.xlsx".to_string(),
            properties: None,
            styles: HashMap::new(),
            sheets: vec![
                Worksheet {
                    name: "Sheet1".to_string(),
                    cells: vec![],
                    merges: vec![],
                    tables: vec![],
                    data_validations: vec![],
                    conditional_formats: vec![
                        ConditionalFormat {
                            range: RangeSpec::A1("A1:A10".to_string()),
                            format_type: "unknown".to_string(),
                            criteria: None,
                            value: None,
                            style: None,
                        },
                    ],
                    sparklines: vec![],
                }
            ],
        };
        
        let result = generator.generate(&dsl);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_conditional_format_cell_without_value() {
        let mut generator = ExcelGenerator::new();
        
        let dsl = ExcelDsl {
            filename: "test_cf_no_value.xlsx".to_string(),
            properties: None,
            styles: HashMap::new(),
            sheets: vec![
                Worksheet {
                    name: "Sheet1".to_string(),
                    cells: vec![],
                    merges: vec![],
                    tables: vec![],
                    data_validations: vec![],
                    conditional_formats: vec![
                        ConditionalFormat {
                            range: RangeSpec::A1("A1:A10".to_string()),
                            format_type: "cell".to_string(),
                            criteria: Some(">".to_string()),
                            value: None,
                            style: None,
                        },
                    ],
                    sparklines: vec![],
                }
            ],
        };
        
        let result = generator.generate(&dsl);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_conditional_format_with_nonexistent_style() {
        let mut generator = ExcelGenerator::new();
        
        let dsl = ExcelDsl {
            filename: "test_cf_no_style.xlsx".to_string(),
            properties: None,
            styles: HashMap::new(),
            sheets: vec![
                Worksheet {
                    name: "Sheet1".to_string(),
                    cells: vec![],
                    merges: vec![],
                    tables: vec![],
                    data_validations: vec![],
                    conditional_formats: vec![
                        ConditionalFormat {
                            range: RangeSpec::A1("A1:A10".to_string()),
                            format_type: "cell".to_string(),
                            criteria: Some(">".to_string()),
                            value: Some(json!(50)),
                            style: Some("nonexistent".to_string()),
                        },
                    ],
                    sparklines: vec![],
                }
            ],
        };
        
        let result = generator.generate(&dsl);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_generate_complex_excel() {
        let mut generator = ExcelGenerator::new();
        
        let mut styles = HashMap::new();
        styles.insert("header".to_string(), Style {
            font: Some(FontStyle {
                bold: Some(true),
                italic: Some(false),
                color: Some("#FFFFFF".to_string()),
                size: Some(12.0),
            }),
            fill: Some(FillStyle {
                color: "#4472C4".to_string(),
            }),
            align: Some(AlignStyle {
                h: Some("center".to_string()),
                v: Some("vcenter".to_string()),
                text_wrap: Some(true),
            }),
            border: Some(BorderStyle {
                around: Some(1),
            }),
            protect: Some(ProtectStyle {
                locked: true,
            }),
        });
        
        styles.insert("highlight".to_string(), Style {
            font: Some(FontStyle {
                bold: Some(true),
                italic: None,
                color: Some("#FF0000".to_string()),
                size: None,
            }),
            fill: None,
            align: None,
            border: None,
            protect: None,
        });
        
        let dsl = ExcelDsl {
            filename: "test_complex.xlsx".to_string(),
            properties: Some(DocumentProperties {
                title: Some("Complex Test".to_string()),
                author: Some("Test Author".to_string()),
                company: Some("Test Co".to_string()),
            }),
            styles,
            sheets: vec![
                Worksheet {
                    name: "Data".to_string(),
                    cells: vec![
                        Cell {
                            r: 0,
                            c: 0,
                            cell_type: CellType::String,
                            value: CellValue::String("Name".to_string()),
                            style: Some("header".to_string()),
                        },
                        Cell {
                            r: 0,
                            c: 1,
                            cell_type: CellType::String,
                            value: CellValue::String("Value".to_string()),
                            style: Some("header".to_string()),
                        },
                        Cell {
                            r: 1,
                            c: 0,
                            cell_type: CellType::String,
                            value: CellValue::String("Item 1".to_string()),
                            style: None,
                        },
                        Cell {
                            r: 1,
                            c: 1,
                            cell_type: CellType::Number,
                            value: CellValue::Number(100.0),
                            style: None,
                        },
                    ],
                    merges: vec![
                        RangeSpec::A1("A10:B10".to_string()),
                    ],
                    tables: vec![],
                    data_validations: vec![
                        DataValidation {
                            range: RangeSpec::Coords(RangeCoords {
                                r1: 1,
                                c1: 0,
                                r2: 10,
                                c2: 0,
                            }),
                            validation_type: "list".to_string(),
                            value: json!(["Item 1", "Item 2", "Item 3"]),
                        },
                    ],
                    conditional_formats: vec![
                        ConditionalFormat {
                            range: RangeSpec::A1("B2:B11".to_string()),
                            format_type: "cell".to_string(),
                            criteria: Some(">".to_string()),
                            value: Some(json!(50.0)),
                            style: Some("highlight".to_string()),
                        },
                    ],
                    sparklines: vec![],
                },
            ],
        };
        
        let result = generator.generate(&dsl);
        assert!(result.is_ok());
        
        let buffer = result.unwrap();
        assert!(!buffer.is_empty());
        assert_eq!(&buffer[0..2], &[0x50, 0x4B]); // ZIP signature
    }
}
