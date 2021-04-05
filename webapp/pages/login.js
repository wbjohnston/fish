import { useState } from 'react'
import Layout from '../components/Layout';
import { login } from '../lib/api'

import { Form, Input, Button, Checkbox } from 'antd';
import { useRouter } from 'next/router';

export default function LoginPage() {
    const router = useRouter()

    function handleLoginSubmit({ username, password }) {
        login(username, password).then(x => {
            router.push("/game")
        }).catch(x => console.error(x))
    }

    const onFinishFailed = (errorInfo) => {
        console.log('Failed:', errorInfo);
    };

    return <Layout>
        <h1>Login</h1>
        <Form
            name="basic"
            initialValues={{ remember: true }}
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

            <Form.Item name="remember" valuePropName="checked">
                <Checkbox>Remember me</Checkbox>
            </Form.Item>

            <Form.Item >
                <Button type="primary" htmlType="submit">
                    Submit
        </Button>
            </Form.Item>
        </Form>
    </Layout>
}
