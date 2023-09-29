param (
  [Parameter(Mandatory=$true, Position=0)]
  [string]$img
)
function Copy-ImageToClipboard {
  param (
      [Parameter(Mandatory=$true, Position=0)]
      [string]$imagePath
  )

  # base64方式和url方式都存在个别图片无法粘贴的现象
  # 测试发现是路径长度受限 
  # 如C:\Users\a\Documents\meme\.asset\normal\？\hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh.jpg可以 
  # 文件名再加个字符就不行了 共78个字节? (gbk 英文1 符号2 汉字3 不常见汉字4)
  
  # 读取图片文件并转换为base64编码的数据URL
  $imageContent = [System.IO.File]::ReadAllBytes($imagePath)
  $base64Data = [System.Convert]::ToBase64String($imageContent)
  $dataUrl = "data:image/gif;base64,$base64Data"
  # $dataUrl = "$url"

  # 创建HTML片段 参考如下
  # https://q.cnblogs.com/q/9115/
  # https://www.zhihu.com/question/460459199
  $htmlCode = @"
  Version:0.9
  StartHTML:00000176
  EndHTML:00000326
  StartFragment:00000210
  EndFragment:00000290
  SourceURL:file:///$imagePath
  <html>
  <body><img src="$dataUrl"></body>
  </html>
"@

  # 将HTML片段复制到剪贴板
  Add-Type -AssemblyName System.Windows.Forms
  [System.Windows.Forms.Clipboard]::SetText($htmlCode, [System.Windows.Forms.TextDataFormat]::Html)
}

# 将本地图片复制到剪贴板
Copy-ImageToClipboard -imagePath $img
