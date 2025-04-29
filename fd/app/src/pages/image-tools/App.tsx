import './App.less';
import {
  TabBar,
  Card,
  ImageUploader,
  Selector,
  Button,
  Space,
} from 'antd-mobile';
import { UnorderedListOutline } from 'antd-mobile-icons';
import { useCallback, useEffect, useRef, useState } from 'react';
import { uploadImage } from './upload.ts';
const tabs = [
  // {
  //   key: 'key1',
  //   title: '一般格式转换',
  //   icon: <AppOutline />,
  // },
  {
    key: 'key2',
    title: 'heic图片转换',
    icon: <UnorderedListOutline />,
  },
  // {
  //   key: 'key3',
  //   title: 'webp动画gif互转',
  //   icon: <GiftOutline />,
  // },
];
const ImageTools = () => {
  const [tab, setTab] = useState<string>('key2');
  const [type, setType] = useState(['jpg']);
  const fileRef = useRef<File>();
  const downloadUrl = useRef<string>();
  useEffect(() => {
    downloadUrl.current = '';
  }, [type]);
  const [loading, setLoading] = useState(false);
  const download = useCallback(() => {
    if (!downloadUrl.current || !fileRef.current) return;
    const a = document.createElement('a');
    a.download = fileRef.current.name + '.' + type[0];
    a.href = downloadUrl.current;
    a.click();
    a.remove();
  }, [type]);
  return (
    <div className="img-tool">
      <TabBar activeKey={tab} onChange={setTab}>
        {tabs.map((item) => (
          <TabBar.Item key={item.key} icon={item.icon} title={item.title} />
        ))}
      </TabBar>
      {tab === 'key2' && (
        <div>
          <Card title="上传heic图片">
            <ImageUploader
              accept="image/heic"
              style={{ objectFit: 'contain' }}
              maxCount={1}
              upload={(file) => {
                fileRef.current = file;
                return uploadImage(file, type[0] as any).then((res) => {
                  downloadUrl.current = res.url;
                  return res;
                });
              }}
            />
          </Card>
          <Card title="选择要转换的格式">
            <Selector
              value={type}
              onChange={setType}
              options={[
                { label: 'jpg', value: 'jpg' },
                { label: 'png', value: 'png' },
                { label: 'webp', value: 'webp' },
              ]}
            />
          </Card>
          <Card>
            <Space>
              <Button
                loading={loading}
                onClick={() => {
                  if (!downloadUrl.current) {
                    if (fileRef.current) {
                      setLoading(true);
                      uploadImage(fileRef.current, type[0] as any)
                        .then((res) => {
                          downloadUrl.current = res.url;
                          download();
                        })
                        .finally(() => {
                          setLoading(false);
                        });
                    }
                    return;
                  }
                  download();
                }}
              >
                下载
              </Button>
            </Space>
          </Card>
        </div>
      )}
    </div>
  );
};

export default ImageTools;
