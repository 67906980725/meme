export class NetUtil {
  static debounce(fn: Function, wait: number) {
    let timeout: any = null
    return function () {
      if (timeout !== null) clearTimeout(timeout)
      timeout = setTimeout(() => fn(), wait)
    }
  }
  static dyn_debounce(wait: number) {
    let timeout: any = null
    return function (fn: Function) {
      if (timeout !== null) clearTimeout(timeout)
      timeout = setTimeout(() => fn(), wait)
    }
  }
}

export class DeviceUtil {
  static get_is_mobile() {
    const reg_r = navigator.userAgent.match(
      /(phone|pad|pod|iPhone|iPod|ios|iPad|Android|Mobile|BlackBerry|IEMobile|MQQBrowser|JUC|Fennec|wOSBrowser|BrowserNG|WebOS|Symbian|Windows Phone)/i
    )
    return reg_r && reg_r.length
  }

  static isMobile = DeviceUtil.get_is_mobile()
}

export class DateUtil {
  static now(): string {
    const date = new Date()
    let month: string | number = date.getMonth() + 1
    let strDate: string | number = date.getDate()

    if (month <= 9) {
      month = '0' + month
    }

    if (strDate <= 9) {
      strDate = '0' + strDate
    }

    return (
      date.getFullYear() +
      '-' +
      month +
      '-' +
      strDate +
      ' ' +
      date.getHours() +
      ':' +
      date.getMinutes() +
      ':' +
      date.getSeconds()
    )
  }
}
