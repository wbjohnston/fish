import {
  Button, Input, notification, Table,
  Form,
} from 'antd';
import { useRouter } from 'next/router';
import React, { useState } from 'react';
import { createGame, listGames } from '../../lib/api';

import Layout from '../../components/AuthedLayout';

export default function ListGamePage({ initialGames }) {
  const router = useRouter();
  const [games, setGames] = useState(initialGames);

  function handleNewGameSubmit({ name }) {
    createGame({ name })
      .then((game) => {
        notification.open({
          message: `successfully created game '${game.name}'`,
        });
      })
      .then(() => {
        listGames().then(setGames);
      });
  }

  function onRow(row) {
    return {
      onClick: () => {
        router.push(`/game/${row.id}`);
      },
    };
  }

  const columns = [
    {
      title: 'Name',
      key: 'name',
      dataIndex: 'name',
    },
  ];

  return (
    <Layout activeNavKey="games" title="fish | games">
      <h2>Create a game</h2>
      <Form onFinish={handleNewGameSubmit}>
        <Form.Item
          label="Name"
          name="name"
        >
          <Input />
        </Form.Item>
        <Form.Item>
          <Button type="primary" htmlType="submit">
            Submit
          </Button>
        </Form.Item>
      </Form>
      <h2>Current Games</h2>
      <Table onRow={onRow} dataSource={games} columns={columns} />
    </Layout>
  );
}

export async function getServerSideProps() {
  const initialGames = await listGames();

  return {
    props: {
      initialGames,
    },
  };
}
