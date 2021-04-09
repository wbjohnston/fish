import React, { useEffect } from 'react';
import {
  Form, Input, Button, notification,
} from 'antd';
import { useRouter } from 'next/router';
import Layout from '../components/Layout';
import useAuth from '../lib/hooks/UseAuth';

export default function LoginPage() {
  const { user, signin } = useAuth();
  const router = useRouter();

  function handleLoginSubmit({ username, password }) {
    signin(username, password)
      .then((x) => {
        notification.open({
          message: 'Succesfully logged in',
        });
      })
      .then((x) => {
        router.push('/game');
      });
  }

  useEffect(() => {
    if (user && router.isReady) {
      router.push('/game');
    }
  }, [router.isReady]);

  const onFinishFailed = (errorInfo) => {
    console.log('Failed:', errorInfo);
  };

  return (
    <Layout>
      <h1>Login</h1>
      <Form
        name="basic"
        onFinish={handleLoginSubmit}
        onFinishFailed={onFinishFailed}
      >
        <Form.Item
          label="Username"
          name="username"
          rules={[{ required: true, message: 'Please input your username!' }]}
        >
          <Input />
        </Form.Item>

        <Form.Item
          label="Password"
          name="password"
          rules={[{ required: true, message: 'Please input your password!' }]}
        >
          <Input.Password />
        </Form.Item>

        <Form.Item>
          <Button type="primary" htmlType="submit">
            Submit
          </Button>
        </Form.Item>
      </Form>
    </Layout>
  );
}
