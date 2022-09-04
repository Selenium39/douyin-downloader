use regex::Regex;

#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct VideoInfo{
    title:String,
    ratio:String,
    cover:String,
    url:String,
    id:String
}

#[tauri::command]
pub async fn parse_dy(url: &str,parse_type:&str) ->  Result<Vec<VideoInfo>, String> {
    let mut videos = Vec::new();
    if parse_type=="video"{
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
        let video_obj = resp["item_list"][0].clone();
        videos.push(video_obj);
    }else{
       // 获取用户uid
       let uid= Regex::new(r#"https://www.douyin.com/user/([\w-]+)"#).unwrap().captures(url).unwrap().get(1).unwrap().as_str().to_string();
       let raw_info = reqwest::get("https://www.iesdouyin.com/web/api/v2/user/info/?sec_uid=".to_string()+uid.as_str())
                        .await
                        .unwrap()
                        .json::<serde_json::Value>()
                        .await
                        .unwrap();
        let video_count = raw_info["user_info"]["aweme_count"].as_u64().unwrap();
        // 获取用户视频列表
        let raw_videos_info = reqwest::get(format!("https://www.iesdouyin.com/web/api/v2/aweme/post/?sec_uid={}&count={}&max_cursor=0",uid,video_count))
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();
        let video_objs = raw_videos_info["aweme_list"].as_array().unwrap().clone();
        for video_obj in video_objs{
            videos.push(video_obj);
        }
    }
    let result = videos.iter().map(|video_obj|{
            let video_url = video_obj["video"]["play_addr"]["url_list"][0].as_str().unwrap().to_string();
            // 获取去水印视频地址
            let video_no_wm_url = video_url.replace("playwm", "play");
            VideoInfo {
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
            }
    }).collect();
    Ok(result)
}
