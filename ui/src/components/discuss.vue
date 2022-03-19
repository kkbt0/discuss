<template>
  <el-row :gutter="20">
    <el-col :xs="1" :sm="2" :md="2" :lg="3" :xl="6"
      ><div class="grid-content bg-purple"
    /></el-col>
    <el-col :xs="22" :sm="20" :md="20" :lg="18" :xl="12">
      <div class="hello">
        <el-link @click="handleChange(1)"
          ><h1>{{ msg }}</h1></el-link
        >
        <el-divider />
        <h2>主题</h2>
        <el-input-number
          v-model="num"
          :min="1"
          :max:="1000"
          @change="handleChange(this.num)"
        />
        <p>
          <b>{{ dis_main.id }} {{ dis_main_user_info.show_name }}</b
          ><br />
          <br />
        </p>
        <p v-html="dis_main.content"></p>
        <div v-if="dis_main.son_nodes != ''">
          <div style="color: gray">回复信息id: {{ dis_main.son_nodes }}</div>
        </div>
        <h2>回复</h2>
        <div v-if="child_dis == 0"><el-empty description="无回复" /></div>
        <div v-for="item in child_dis" :key="item.id">
          {{ item.id }}<b>{{ item.author }}</b
          >&nbsp;&nbsp;to&nbsp;&nbsp;{{ item.reply_to }}&nbsp;&nbsp;
          <el-link @click="detail(item.id)" type="primary">详细</el-link
          >&nbsp;&nbsp;
          <el-link @click="delete_discussion(item.id)" type="danger"
            >删除</el-link
          >
          <br />
          <p v-html="item.content"></p>
          <div v-if="item.son_nodes != ''">
            <el-link @click="handleChange(item.id)">查看回复</el-link>
          </div>
          <div><br /></div>
        </div>
        <br />
        <el-input v-model="user_key" placeholder="请输入用户key" clearable />
        <el-input
          v-model="textarea"
          placeholder="输入回复内容"
          :autosize="{ minRows: 5, maxRows: 1000 }"
          maxlength="1000"
          type="textarea"
        />
        <div style="margin: 100px 0" />
        <el-button @click="post_discussion">新增</el-button>
        <el-button @click="get_discussion(this.num)">刷新</el-button>
      </div>
    </el-col>
    <el-col :xs="1" :sm="2" :md="20" :lg="3" :xl="6"
      ><div class="grid-content bg-purple"
    /></el-col>
  </el-row>
</template>
<script>
import axios from "axios";
import { ElMessage } from "element-plus";
export default {
  name: "HelloWorld",
  props: {
    msg: String,
  },
  mounted() {
    this.get_discussion(1);
  },

  data() {
    return {
      dis_main_user_info: {
        id: 1,
        show_name: "11",
        status: 0,
      },
      dis_main: {
        id: 1,
        author: 1,
        content: "Welcome to the world!",
        created_at: "0",
        up: 0,
        down: 0,
        read_number: 0,
        sharded_number: 0,
        reply_to: 0,
        father_nodes: 0,
        son_nodes: "Null",
      },
      main_post_author_id: 1,
      baseurl: "http://127.0.0.1:8000",
    //  baseurl: "https://api.ftls.xyz",
      child_dis: 0,
      textarea: "Hello world!",
      author: "kkk",
      textarea3: 1,
      num: 1,
      user_key: "guest",
    };
  },
  methods: {
    jump_to_index() {
      this.num = 1;
    },
    get_discussion(x) {
      axios.get(this.baseurl + "/discussion/" + x).then((res) => {
        this.dis_main = res.data.main_dis;
        this.main_post_author_id = res.data.main_dis.author;

        this.child_dis = res.data.child_dis;

        console.log("child_dis is");
        console.log(res.data.child_dis);
        console.log(this.child_dis);
        console.log(res.data);

        this.get_user_by_id(this.dis_main.author);

        var map = new Map();
        this.child_dis.forEach(function (item) {
          map.set(item.author, "空");
        });

        var use = this;
        function update2() {
          use.child_dis.forEach(function (item) {
            item.author = map.get(item.author);
          });
        }
        var baseurl = this.baseurl;
        function update() {
          for (let key of map.keys()) {
            axios.get(baseurl + "/userid/" + key).then((res2) => {
              map.set(key, res2.data.show_name);
            });
          }
          return 0;
         // update2();
          //clearTimeout(timer);
        }
        update();
        setTimeout(update2, 1000)

      });
    },
    post_discussion() {
      if (this.textarea == "") {
        ElMessage({
          showClose: true,
          message: "消息不能为空",
          type: "error",
        });
        return;
      }
      var data = {
        father_node: this.num,
        content: this.textarea,
        author: this.user_key,
        reply_to: this.main_post_author_id, // str
      };
      console.log(data);
      axios
        .post(this.baseurl + "/dispost", data, {
          timeout: 1000,
          headers: { "Content-Type": "application/json" },
        })
        .then((res) => {
          console.log(res);
          this.textarea = "";
          this.get_discussion(this.num);
        });
    },
    delete_discussion(x) {
      axios.delete(this.baseurl + "/dis_sign_del/" + x).then((res) => {
        console.log(res);
        this.get_discussion(this.num);
      });
    },
    get_user_by_id(x) {
      axios.get(this.baseurl + "/userid/" + x).then((res) => {
        this.dis_main_user_info = res.data;
        this.author = this.dis_main_user_info.show_name;
      });
    },
    get_user_by_id_return(x) {
      var tem;
      axios.get(this.baseurl + "/userid/" + x).then((res) => {
        tem = res.data.show_name;
      });
      return tem;
    },
    handleChange(x) {
      console.log(x);
      this.get_discussion(x);
    },
    detail(x) {
      this.num = x
      this.handleChange(x)
    }
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #006eff;
}
</style>
