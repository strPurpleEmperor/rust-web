import './App.css';
import { useEffect, useState } from 'react';
import request from '../../utils/request.ts';

const User = () => {
  const [userList, setUserList] = useState([]);
  useEffect(() => {
    request.get('/api/users').then((res: any) => {
      setUserList(res);
    });
  }, []);
  return (
    <div>
      <pre>{JSON.stringify(userList, null, 2)}</pre>
    </div>
  );
};

export default User;
