import { request } from './axios'
import { Config } from './config'

export class FolderService {

  static path_to_url(path: string) {
    return Config.baseURL + '/file/' + path;
  }

  static async re_init_db() {
    return request('/folder/re_init_db', {}, 'get')
  }
  
  static async styles() {
    return request('/style/list', {}, 'get')
  }
  
  static async by_key(keyword: string, styleId: number) {
    return request('/folder/by_key', { keyword: keyword, styleId: styleId }, 'get')
  }

  static async click(id: number, auto: number) {
    return request('/folder/click', { id: id, auto: auto }, 'get')
  }
  static async open(id: number) {
    return request('/folder/open', { id: id }, 'get')
  }

  static async img_click(id: number, auto: number) {
    return request('/folder/img_click', { id: id, auto: auto }, 'get')
  }

  static async get_imgs(id: number) {
    return request('/folder/get_imgs', { id: id }, 'get')
  }

  static async copy_img(path: string, host: string) {
    return request('/image/copy', { path: path, host: host }, 'get')
  }
  static async add(name: string, keyword: string, styleId: number) {
    return request('/folder/add', { name: name, keyword: keyword, styleId: styleId }, 'post')
  }
  static async add_img(folder_id: number, data: FormData) {
    return request('/img/add?id=' + folder_id, data, 'upload')
  }
}
