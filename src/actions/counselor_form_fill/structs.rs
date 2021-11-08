use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CounselorResponse<T> {
    pub code: String,
    #[serde(default)]
    pub message: String,
    pub datas: T,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CounselorPaginator<T> {
    pub total_size: i64,
    #[serde(default)]
    pub page_size: i64,
    #[serde(default)]
    pub page_number: i64,
    // TODO: figure out what's this
    // #[serde(default)]
    // pub exist_data: i64,
    pub rows: Vec<T>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectorFormInstance {
    pub wid: String,
    pub instance_wid: i64,
    pub form_wid: String,
    pub priority: String,
    pub subject: String,
    pub content: String,
    pub sender_user_name: String,
    pub create_time: String,
    pub start_time: String,
    pub end_time: String,
    pub current_time: String,
    pub is_handled: i64,
    pub is_read: i64,
}

// TODO: a few more optional field
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectorInfo {
    pub wid: String,
    pub instance_wid: i64,
    pub form_wid: String,
    pub priority: String,
    pub end_time: String,
    pub current_time: String,
    pub school_task_wid: String,
    pub is_confirmed: i64,
    pub sender_user_name: String,
    pub create_time: String,
    // pub attachment_urls: Option<String>,
    // pub attachment_names: Option<String>,
    // pub attachment_sizes: Option<String>,
    pub is_user_submit: i64,
    pub fetch_stu_location: bool,
    pub is_location_failed_sub: bool,
    pub address: String,
    pub subject: String,
}

pub enum FormType {
    Survey,
    Exams,
    Votes
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormInfo {
    pub wid: String,
    pub form_type: String,
    pub form_title: String,
    // pub exam_time: i64,
    pub form_content: String,
    // pub back_reason: String,
    // pub is_back: i64,
    // pub attachments: Vec<>
    // pub score: i64,
    // pub stu_score: ,
    pub confirm_desc: String,
    #[serde(rename = "isshowOrdernum")]
    pub is_show_ordernum: i64,
    pub is_anonymous: i64,
    #[serde(rename = "isallowUpdated")]
    pub is_allow_updated: i64,
    #[serde(rename = "isshowScore")]
    pub is_show_score: i64,
    #[serde(rename = "isshowResult")]
    pub is_show_result: i64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormDetail {
    pub collector: CollectorInfo,
    pub form: FormInfo,
}

// 1.文本 2.单选题 3.多选题 4.上传照片 5数字输入 6日期时间 7地址填写 8 量表 9 民族 10 政治面貌 11手机号 12 身份证 13 邮箱地址 14 文字投票 15 图文投票 16 手写签名 17 院系班级 18 学生选择 19判断题 20填空题 21 地图选点 22 政工选择 23备注说明
// #[derive(PartialEq, Debug, Serialize, Deserialize)]
// pub enum FieldType {
//     TextInput = 1,
//     SingleChoice,
//     MultipleChoice,
//     PhotoUpload,
//     NumberInput,
//     DateTime,
//     Address,
//     Matrix, 
//     EthnicGroup,
//     PoliticalStatus,
//     Mobile,
//     IDNumber,
//     Email,
//     TextVote,
//     PhotoVote,
//     Signature,
//     // ... useless items
// }

