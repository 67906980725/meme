import axios from 'axios'
import { showMessage } from './status'
import { useMessage } from 'naive-ui'
import { Config } from './config'

const message = useMessage()

axios.defaults.timeout = 60000

// axios.defaults.baseURL = import.meta.env.VITE_APP_BASE_API
axios.defaults.baseURL = Config.baseURL

// request 拦截器
axios.interceptors.request.use(
  (config) => {
    if (!config.headers) {
      config.headers = {}
    }
    if (!config.headers['Content-Type']) {
      config.headers['Content-Type'] = 'application/json;charset=UTF-8' // 传参方式-json
    }
    // config.headers = {
    //   //'Content-Type':'application/x-www-form-urlencoded',   // 传参方式-表单
    //   'Content-Type': 'application/json;charset=UTF-8' // 传参方式-json
    // }
    return config
  },
  (error) => {
    return Promise.reject(error)
  }
)

// response 拦截器
axios.interceptors.response.use(
  (response) => {
    return response ? response.data : response
  },
  (error) => {
    const { response } = error
    if (response) {
      // 请求已发出，但是不在2xx的范围
      showMessage(response.status) // 传入响应码，匹配响应码对应信息
      return Promise.reject(response.data)
    } else {
      message.warning('网络连接异常, 请稍后再试!')
    }
  }
)

// 封装 GET POST 请求并导出
export function request(url = '', params = {}, type = 'POST') {
  // 设置 url params type 的默认值
  return new Promise((resolve, reject) => {
    let promise
    if (type.toUpperCase() == 'GET') {
      promise = axios({
        url,
        params
      })
    } else if (type.toUpperCase() == 'POST') {
      promise = axios({
        method: 'POST',
        url,
        data: params
      })
    } else if (type.toUpperCase() == 'UPLOAD') {
      promise = axios({
        method: 'POST',
        headers: {
          'Content-Type': 'multipart/form-data'
        },
        url,
        data: params
      })
    } else {
      message.warning('请求方式暂未支持:' + type)
      return
    }
    // 处理返回
    promise
      .then((res) => {
        resolve(res)
      })
      .catch((err) => {
        reject(err)
      })
  })
}
