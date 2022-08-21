<template>
  <div id="app">
    <div id="form">
      <el-row style="padding: 20px 0" type="flex" justify="space-around">
        <el-col :span="5">
          <el-select style="width: 100%" v-model="form.type">
            <el-option label="单个视频" value="single"></el-option>
            <el-option label="用户所有视频" value="multi"></el-option>
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
       
    </div>
  </div>
</template>

<script>
const invoke = window.__TAURI__.invoke;
export default {
  data() {
    return {
      form: {
        type: "single",
        url: "8.25 CHI:/ look 别发呆 %看我造型say哇塞  https://v.douyin.com/Fpv7GVp/ 复制此链接，打开Dou音搜索，直接观看视频！",
      },
    };
  },
  methods: {
    parse() {
      invoke("parse_dy", this.form).then((res) => {
        console.log("解析结果");
        console.log(res);
      });
    },
  },
};
</script>

<style scoped>
</style>