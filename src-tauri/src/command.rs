use regex::Regex;

#[derive(serde::Serialize)]
pub struct VideoInfo{
    title:String,
    ratio:String,
    cover:String,
    url:String,
    id:String
}

#[tauri::command]
pub async fn parse_dy(url: &str) ->  Result<VideoInfo, String> {
    // 获请求抖音链接中的url
    let short_url = Regex::new(r"https://\S*")
        .unwrap()
        .captures(url)
        .unwrap()
        .get(0)
        .unwrap()
        .as_str()
        .to_string();
    // 请获取视频唯一id
    let video_url = reqwest::get(&short_url)
        .await
        .unwrap()
        .url()
        .as_str()
        .to_string();
    let video_id = Regex::new(r"video/(\d+)")
        .unwrap()
        .captures(&video_url)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string();
    // 获取视频播放地址
    let resp = reqwest::get("https://www.iesdouyin.com/web/api/v2/aweme/iteminfo/?item_ids=".to_string() + video_id.as_str())
            .await
            .unwrap()
            .json::<serde_json::Value>()
            .await
            .unwrap();
    let video_obj = &resp["item_list"][0];
    let video_url = video_obj["video"]["play_addr"]["url_list"][0].as_str().unwrap().to_string();
    // 获取去水印视频地址
    let video_no_wm_url = video_url.replace("playwm", "play");
    Ok(VideoInfo {
        title: video_obj["desc"]
            .as_str()
            .unwrap_or("")
            .to_string(),
        ratio: video_obj["video"]["ratio"]
            .as_str()
            .unwrap_or("")
            .to_string(),
        cover: video_obj["video"]["cover"]["url_list"][0]
            .as_str()
            .unwrap_or("")
            .to_string(),
        id: video_obj["aweme_id"].as_str().unwrap_or("").to_string(),
        url:video_no_wm_url,
    })
}
