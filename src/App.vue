<template>
  <div id="app">
    <div id="form">
      <el-row style="padding: 20px 0" type="flex" justify="space-around">
        <el-col :span="5">
          <el-select style="width: 100%" v-model="form.parseType">
            <el-option label="单个视频" value="video"></el-option>
            <el-option label="用户所有视频" value="user"></el-option>
          </el-select>
        </el-col>
        <el-col :span="15">
          <el-input
            v-model="form.url"
            placeholder="请填入分享的视频链接"
          ></el-input>
        </el-col>
        <el-col :span="2">
          <el-button type="primary" @click="parse">解析</el-button>
        </el-col>
      </el-row>
    </div>
    <div id="list">
      <el-table
        :data="list"
        style="width: 100%"
        :header-cell-style="{ 'text-align': 'center' }"
      >
        <el-table-column align="center" prop="id" label="序号">
        </el-table-column>
        <el-table-column align="center" prop="cover" label="封面">
          <template slot-scope="scope">
            <el-image
              style="width: 100px; height: 100px"
              :src="scope.row.cover"
              fit="cover"
            ></el-image>
          </template>
        </el-table-column>
        <el-table-column align="center" prop="title" label="标题">
        </el-table-column>
        <el-table-column align="center" prop="ratio" label="分辨率">
        </el-table-column>
        <el-table-column label="操作" align="center">
          <template slot-scope="scope">
            <el-button
              @click="download(scope.row)"
              :loading="scope.row.downloading"
              type="primary"
              size="small"
              ><i class="el-icon-download"></i>下载</el-button
            >
          </template>
        </el-table-column>
      </el-table>
    </div>
  </div>
</template>

<script>
import { writeBinaryFile, BaseDirectory } from "@tauri-apps/api/fs";
import { fetch, ResponseType } from "@tauri-apps/api/http";
const invoke = window.__TAURI__.invoke;
export default {
  data() {
    return {
      form: {
        parseType: "video",
        // url: "8.25 CHI:/ look 别发呆 %看我造型say哇塞  https://v.douyin.com/Fpv7GVp/ 复制此链接，打开Dou音搜索，直接观看视频！",
        url:"https://www.douyin.com/user/MS4wLjABAAAACrxUoQvX2JVH3NeVKvErEEG59BuU5hs_Z-rH9dP0tH4"
      },
      list: []
    };
  },
  methods: {
    parse() {
      invoke("parse_dy", this.form).then((res) => {
        res.forEach(item=>{
          item.downloading = false
        })
        this.list = res;
      });
    },
    async download(video) {
      video.downloading=true
      const res = await fetch(video.url, {
        method: "GET",
        timeout: 10000,
        responseType: ResponseType.Binary,
      }); 
      await writeBinaryFile(`${video.id}.mp4`, res.data,{dir:BaseDirectory.Download})
      video.downloading = false
      this.$message({
          message: '视频已下载',
          type: 'success'
      });
    },
  },
};
</script>

<style scoped>
</style>