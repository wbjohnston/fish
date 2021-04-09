import React, { useState, useEffect } from 'react';

import { useRouter } from 'next/router';
import { Table } from 'antd';
import { listUsers } from '../../lib/api';
import Layout from '../../components/AuthedLayout';

export default function ListUserPage() {
  const router = useRouter();
  const [users, setUsers] = useState([]);

  useEffect(() => {
    listUsers().then((x) => setUsers(x));
  }, []);

  function onRow(row) {
    return {
      onClick: () => {
        router.push(`/user/${row.id}`);
      },
    };
  }

  const columns = [
    {
      title: 'Username',
      key: 'username',
      dataIndex: 'username',
    },
  ];

  return (
    <Layout activeNavKey="games" title="fish | games">
      <h2>Current Users</h2>
      <Table onRow={onRow} dataSource={users} columns={columns} />
    </Layout>
  );
}
