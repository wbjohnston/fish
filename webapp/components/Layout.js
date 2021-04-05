import { Layout as ALayout, Menu } from 'antd'
import { Link } from 'next/link'



export default function Layout({ children, activeNavKey, title }) {
    return <ALayout classname="layout" title={title}>
        <ALayout.Header>
            <Menu activeKey={activeNavKey} theme="dark" mode="horizontal">
                <Menu.Item key="clients">clients</Menu.Item>
                <Menu.Item key="users">users</Menu.Item>
                <Menu.Item key="games">games</Menu.Item>
            </Menu>
        </ALayout.Header>
        <ALayout.Content style={{ padding: '4rem' }}>
            {children}
        </ALayout.Content>
    </ALayout>
}
