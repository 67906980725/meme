import { h, nextTick } from 'vue'
import { NButton, NAvatar } from 'naive-ui'

import { DateUtil } from './util'

// main.vue
{/* <template>
  <n-notification-provider>
    <n-message-provider>
      <HomeView />
    </n-message-provider>
  </n-notification-provider>
</template> */}

// HomeVue.vue
// import { useNotification, useMessage } from 'naive-ui'
// export default {
//   setup() {
//     window.$notification = useNotification()
//     window.$message = useMessage()
//   }
// }

export function notify(title, content, okContent, confirmOkMsg) {
  let markAsRead = false

  nextTick(() => {
    const n = window.$notification.create({
      title: title,
      content: content,
      meta: DateUtil.now(),
      action: () =>
        h(
          NButton,
          {
            text: true,
            type: 'primary',
            onClick: () => {
              markAsRead = true
              n.destroy()
            }
          },
          { default: () => okContent }
        ),
      onClose: () => {
        if (confirmOkMsg && !markAsRead) {
          window.$message.warning(confirmOkMsg)
          return false
        }
      }
    })
  })
}
