import { Layout as ALayout, Menu, notification } from 'antd';
import { UserOutlined } from '@ant-design/icons';
import SubMenu from 'antd/lib/menu/SubMenu';
import Link from 'next/link';
import React, { useEffect } from 'react';
import useAuth from '../lib/hooks/UseAuth';
import useWebsocket from '../lib/hooks/UseWebsocket';

function NavMenu({ activeNavKey }) {
  const { user } = useAuth();

  if (!user) {
    return (
      <Menu style={{ float: 'right', background: 'inherit' }} theme="dark" mode="horizontal">
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
    );
  }

  return (
    <>
      <Menu style={{ float: 'left', background: 'inherit' }} activeKey={activeNavKey} theme="dark" mode="horizontal">
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
      </Menu>
      <Menu theme="dark" mode="horizontal" style={{ float: 'right', background: 'inherit' }}>
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
    </>
  );
}

export default function Layout({ children, activeNavKey, title }) {
  const { websocket } = useWebsocket();
  const { user } = useAuth();
  useEffect(() => {
    if (!websocket) {
      return;
    }

    function handleMessage(e) {
      notification.open({
        message: e.data,
      });
    }

    websocket.addEventListener('message', handleMessage);
    return () => {
      websocket.removeEventListener('message', handleMessage);
    };
  }, [websocket]);
  return (
    <ALayout style={{ minHeight: '100vh' }} className="layout" title={title}>
      <ALayout.Header style={{ background: user?.role === 'admin' ? '#27ae60' : '#2980b9' }}>
        <NavMenu activeNavKey={activeNavKey} />
      </ALayout.Header>
      <ALayout.Content style={{ padding: '4rem' }}>
        {children}
      </ALayout.Content>
    </ALayout>
  );
}
