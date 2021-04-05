import { Layout as ALayout, Menu } from 'antd'
import { UserOutlined } from '@ant-design/icons'
import SubMenu from 'antd/lib/menu/SubMenu';
import Link from 'next/link'
import React from "react"
import useAuth from '../lib/hooks/UseAuth'



function NavMenu({ activeNavKey }) {
    const { user } = useAuth();
    if (!user) {
        return <Menu theme="dark" mode="horizontal">
            <Menu.Item key="login">
                <Link href="/login">
                    <a>Login</a>
                </Link>
            </Menu.Item>
            <Menu.Item key="register">
                <Link href="/register">
                    <a>Register</a>
                </Link>
            </Menu.Item>
        </Menu>
    }


    return <Menu activeKey={activeNavKey} theme="dark" mode="horizontal">
        <Menu.Item key="users">
            <Link href="/user" passHref>
                <a>Users</a>
            </Link>
        </Menu.Item>
        <Menu.Item key="game">
            <Link href="/game" passHref>
                <a>Games</a>
            </Link>
        </Menu.Item>
        <SubMenu key="user" title={user.username} icon={<UserOutlined />}>
            <Menu.Item key="profile">
                <Link href="/me" passHref>
                    <a>Profile</a>
                </Link>
            </Menu.Item>
            <Menu.Item key="logout">
                <Link href="/logout" passHref>
                    <a>Logout</a>
                </Link>
            </Menu.Item>
        </SubMenu>
    </Menu>
}


export default function Layout({ children, activeNavKey, title }) {
    return <ALayout className="layout" title={title}>
        <ALayout.Header>
            <NavMenu activeNavKey={activeNavKey} />
        </ALayout.Header>
        <ALayout.Content style={{ padding: '4rem' }}>
            {children}
        </ALayout.Content>
    </ALayout >
}
