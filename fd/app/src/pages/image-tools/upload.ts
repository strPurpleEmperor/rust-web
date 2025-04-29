import request from '../../utils/request.ts';

export function uploadImage(
  file: File,
  type?: 'jpg' | 'png' | 'webp',
): Promise<{ url: string }> {
  const form = new FormData();
  form.set('file', file);
  form.set('t_type', type ?? 'jpg');
  return request
    .post('/api/convert', form, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
      responseType: 'arraybuffer',
    })
    .then((res: any) => {
      const blob = new Blob([res as Uint8Array], {
        type: `image/${type ?? 'jpg'}`,
      });
      const url = URL.createObjectURL(blob);
      console.log(url);
      return { url };
    });
}
