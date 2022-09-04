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
            :placeholder="
              form.parseType === 'video'
                ? '8.25 CHI:/ look 别发呆 %看我造型say哇塞  https://v.douyin.com/Fpv7GVp/ 复制此链接，打开Dou音搜索，直接观看视频！'
                : 'https://www.douyin.com/user/MS4wLjABAAAACrxUoQvX2JVH3NeVKvErEEG59BuU5hs_Z-rH9dP0tH4'
            "
          ></el-input>
        </el-col>
        <el-col :span="2">
          <el-button size="small" type="primary" @click="parse">解析</el-button>
        </el-col>
      </el-row>
    </div>
    <div id="list">
      <el-row>
        <el-button @click="batchDownload" size="small" type="primary"
          >批量下载</el-button
        >
      </el-row>
      <el-table
        ref="videoTable"
        :data="list"
        style="width: 100%"
        :header-cell-style="{ 'text-align': 'center' }"
        @selection-change="handleSelectionChange"
      >
        <el-table-column type="selection" width="55"> </el-table-column>
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
            <el-button @click="preview(scope.row)" type="primary" size="small"
              ><i class="el-icon-video-play"></i>预览</el-button
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
        url: "",
      },
      list: [],
      selections: [],
    };
  },
  methods: {
    parse() {
      if (!this.form.url) {
        this.form.url =
          this.form.parseType === "video"
            ? "https://v.douyin.com/Fpv7GVp/"
            : "https://www.douyin.com/user/MS4wLjABAAAACrxUoQvX2JVH3NeVKvErEEG59BuU5hs_Z-rH9dP0tH4";
      }
      invoke("parse_dy", this.form).then((res) => {
        res.forEach((item) => {
          item.downloading = false;
        });
        this.list = res;
      });
    },
    async download(video) {
      video.downloading = true;
      const res = await fetch(video.url, {
        method: "GET",
        timeout: 10000,
        responseType: ResponseType.Binary,
      });
      await writeBinaryFile(`${video.id}.mp4`, res.data, {
        dir: BaseDirectory.Download,
      });
      video.downloading = false;
    },
    async batchDownload() {
      const tasks = [];
      this.selections.forEach((video) => {
        tasks.push(this.download(video));
      });
      await Promise.all(tasks);
      this.$refs.videoTable.clearSelection();
    },
    async preview(video) {
      const el = document.createElement("a");
      el.style.display = "none";
      el.setAttribute("target", "_blank");
      el.href = video.url;
      document.body.appendChild(el);
      el.click();
      document.body.removeChild(el);
    },
    handleSelectionChange(val) {
      this.selections = val;
    },
  },
};
</script>

<style scoped>
</style>