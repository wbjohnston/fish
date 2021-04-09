import React, { useEffect, useState } from 'react';
import {
  Button, Card, Input, Typography, Form, notification,
} from 'antd';
import { useRouter } from 'next/router';

import Layout from '../../components/AuthedLayout';
import { fetchGame } from '../../lib/api';
import useAuth from '../../lib/hooks/UseAuth';
import useWebsocket from '../../lib/hooks/UseWebsocket';
import PokerTable from '../../components/PokerTable';

export default function ListGamePage() {
  const { user } = useAuth();
  const [game, setGame] = useState({});
  const [table, setTable] = useState(null);
  const { websocket } = useWebsocket();
  const router = useRouter();

  useEffect(() => {
    if (!router.isReady) {
      return;
    }

    fetchGame(router.query.id).then((x) => setGame(x));
  }, []);

  function handleTableUpdate(newTable) {
    setTable(newTable);
  }

  useEffect(() => {
    if (!websocket) {
      return;
      // throw error
    }

    function handleWebsocketMessage(e) {
      const message = e.data;

      switch (message.kind) {
        case 'sync':
          handleTableUpdate(message.payload);
          break;
        default:
          notification.open({
            message: 'received message in game screen',
            description: e.data,
          });
          break;
      }
    }

    websocket.addEventListener('message', handleWebsocketMessage);

    // eslint-disable-next-line consistent-return
    return () => websocket.removeEventListener('message', handleWebsocketMessage);
  }, [websocket]);

  function sitGame() {
    if (!websocket) {
      // throw error
    }

    websocket.send(JSON.stringify({
      gameId: router.query.id,
      action: {
        kind: 'sit',
        options: {},
      },
    }));
  }

  function handleStartGame() {
    if (!websocket) {
      // throw error
    }

    websocket.send(JSON.stringify({
      gameId: router.query.id,
      action: {
        kind: 'start',
      },
    }));
  }

  function leaveGame() {
    if (!websocket) {
      // throw error
    }

    websocket.send(JSON.stringify({
      gameId: router.query.id,
      action: {
        kind: 'leave',
      },
    }));
  }

  function submitFold() {
    if (!websocket) {
      // throw error
    }

    websocket.send(JSON.stringify({
      gameId: router.query.id,
      action: {
        kind: 'fold',
      },

    }));
  }

  function submitBet({ amount }) {
    if (!websocket) {
      // throw error
    }

    websocket.send(JSON.stringify({
      gameId: router.query.id,
      action: {
        kind: 'bet',
        options: {
          amount: parseInt(amount, 10),
        },
      },

    }));
  }

  function joinGame() {
    if (!websocket) {
      // throw error
    }

    websocket.send(JSON.stringify({
      gameId: router.query.id,
      action: {
        kind: 'join',
      },
    }));
  }

  return (
    <Layout>
      <h1>
        Game:
        {game.name}
      </h1>

      <Button onClick={joinGame} color="green">Join Game</Button>
      <Button onClick={sitGame} color="green">Sit</Button>
      <Button onClick={leaveGame} color="green">Leave</Button>
      <Button onClick={submitFold} color="green">Fold</Button>
      <Button onClick={handleStartGame} color="green">Start</Button>
      <Form onFinish={submitBet}>
        <Form.Item label="Amount" name="amount">
          <Input />
        </Form.Item>
        <Form.Item>
          <Button type="primary" htmlType="submit">
            Submit Bet
          </Button>
        </Form.Item>
      </Form>

      <h2>RAW</h2>
      <Card>
        <pre>
          <Typography code="json">
            {JSON.stringify(game, null, 2)}
          </Typography>
        </pre>
      </Card>

      <PokerTable table={table} />
    </Layout>
  );
}
