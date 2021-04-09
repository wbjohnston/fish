import React from 'react';
import {
  Form, Button, Input, notification,
} from 'antd';
import { useRouter } from 'next/router';
import { register } from '../lib/api';
import Layout from '../components/Layout';

export default function LoginPage() {
  const router = useRouter();

  function handleRegisterSubmit({ username, password }) {
    register(username, password)
      .then(() => {
        notification.open({
          message: 'succesfully registered',
        });
      })
      .then(() => {
        router.push('/login');
      }).catch((x) => console.error(x));
  }

  const onFinishFailed = (errorInfo) => {
    console.log('Failed:', errorInfo);
  };

  return (
    <Layout>
      <h1>Register</h1>
      <Form
        name="basic"
        initialValues={{ remember: true }}
        onFinish={handleRegisterSubmit}
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
