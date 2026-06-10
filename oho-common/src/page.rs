use serde::{Deserialize, Serialize};
/// 分页请求参数
#[derive(Debug, Deserialize)]
pub struct PageParam {
    /// 页码，从 1 开始
    #[serde(default = "default_page")]
    pub page: u64,
    /// 每页条数，默认 10
    #[serde(default = "default_size")]
    pub size: u64,
}
fn default_page() -> u64 {
    1
}
fn default_size() -> u64 {
    10
}
impl PageParam {
    pub fn offset(&self) -> u64 {
        (self.page - 1) * self.size
    }
}
#[derive(Debug, Serialize)]
pub struct PageResponse<T: Serialize> {
    /// 总页数
    pub pages: u64,
    /// 每页条数
    pub size: u64,
    /// 数据总量
    pub total: u64,
    /// 当前页码
    pub current: u64,
    /// 数据内容
    pub records: Vec<T>,
}
impl<T: Serialize> PageResponse<T> {
    pub fn new(total: u64, param: &PageParam, records: Vec<T>) -> Self {
        let pages = total.div_ceil(param.size);
        Self {
            pages,
            size: param.size,
            total,
            current: param.page,
            records,
        }
    }
}
