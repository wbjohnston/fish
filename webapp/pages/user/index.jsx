import React, { useState, useEffect } from 'react';

import { useRouter } from 'next/router';
import { Table } from 'antd';
import { listUsers } from '../../lib/api';
import Layout from '../../components/AuthedLayout';

export default function ListUserPage({ initialUsers }) {
  const router = useRouter();
  const [users, setUsers] = useState(initialUsers);

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

export async function getServerSideProps() {
  const initialUsers = await listUsers();
  return {
    props: { initialUsers },
  };
}
