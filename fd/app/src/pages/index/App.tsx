import './App.css';
import { mirror, mirror_gif, MirrorDirection } from 'wasm';
import styles from './index.module.less';
import { Button, Card, Radio, Space, Toast } from 'antd-mobile';
import { ChangeEvent, useCallback, useState } from 'react';

const App = () => {
  const [isGif, setIsGif] = useState(false);
  const [originUrl, setOriginUrl] = useState<string>();
  const [url, setUrl] = useState<string>();
  const [file, setFile] = useState<File>();
  const [direct, setDirect] = useState<MirrorDirection>(
    MirrorDirection.LeftToRight,
  );
  const uploadFile = useCallback(
    (e: ChangeEvent<HTMLInputElement>, isGif = false) => {
      const file = e.target.files?.[0];
      if (!file) return;
      // 读取文件为 Blob URL
      const blobUrl = URL.createObjectURL(file);
      setOriginUrl(blobUrl);
      setFile(file);
      setIsGif(isGif);
    },
    [],
  );
  const uploadGifFile = useCallback((e: ChangeEvent<HTMLInputElement>) => {
    uploadFile(e, true);
  }, []);
  return (
    <Card className="content" title="镜像图片">
      <Card title="原图片">
        <div className={styles.imgWrap}>
          {originUrl ? <img src={originUrl} alt="" /> : <div>先上传图片</div>}
        </div>
      </Card>
      <Space direction="vertical">
        <Radio.Group onChange={(val: any) => setDirect(val)}>
          <Space wrap>
            <Radio value={MirrorDirection.LeftToRight}>从左往右</Radio>
            <Radio value={MirrorDirection.RightToLeft}>从右往左</Radio>
            <Radio value={MirrorDirection.TopToBottom}>从上往下</Radio>
            <Radio value={MirrorDirection.BottomToTop}>从下往上</Radio>
          </Space>
        </Radio.Group>
        <Space>
          <Button className={styles.btn} color="primary">
            <input
              className={styles.upload}
              type="file"
              accept="image/jpeg,image/png"
              onChange={uploadFile}
            />
            上传jpeg、png
          </Button>
          <Button className={styles.btn} color="primary">
            <input
              className={styles.upload}
              type="file"
              accept="image/gif"
              onChange={uploadGifFile}
            />
            上传gif
          </Button>
        </Space>
        <Space>
          <Button
            color="success"
            onClick={async () => {
              if (!file) return;
              const gifBuffer = await file.arrayBuffer();
              const mirrorFn = isGif ? mirror_gif : mirror;
              const uint8Array = mirrorFn(new Uint8Array(gifBuffer), direct);
              const blob = new Blob([uint8Array], { type: file.type });
              const blobUrl = URL.createObjectURL(blob);
              setUrl(blobUrl);
            }}
          >
            开始转换
          </Button>
          <Button
            color="primary"
            onClick={() => {
              if (!file) return;
              if (!url) {
                Toast.show({ content: '没有图片可以下载' });
                return;
              }
              const link = document.createElement('a');
              link.download = file.name;
              link.href = url;
              // 将链接添加到文档中
              document.body.appendChild(link);

              // 自动触发点击事件
              link.click();

              // 移除链接元素
              document.body.removeChild(link);
            }}
          >
            下载图片
          </Button>
        </Space>
      </Space>
      <Card title="转换后的图片">
        {url && <img className={styles.imgWrap} src={url} alt="" />}
      </Card>
    </Card>
  );
};

export default App;
