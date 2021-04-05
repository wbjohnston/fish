import { Button, Input, Table } from 'antd'
import Layout from '../../components/AuthedLayout'
import { useRouter } from "next/router"
import { useEffect, useState } from 'react';
import { createGame, listGames } from '../../lib/api';
import { Form } from 'antd'

export default function ListGamePage() {
    const router = useRouter();
    const [games, setGames] = useState([]);

    useEffect(() => {
        if (!router.isReady) {
            return
        }

        listGames().then(setGames)

    }, [])

    function onRow(row) {
        return {
            onClick: () => {
                router.push(`/game/${row.id}`)
            }
        }
    }

    const columns = [
        {
            title: "Name",
            key: 'name',
            dataIndex: 'name'
        },
        {
            title: "Action",
            key: 'action',
            render: (text, record) => {
                return <a>Join</a>
            }
        }
    ]


    return <Layout activeNavKey="games" title="fish | games">
        <h2>Create a game</h2>
        <Form onFinish={({ name }) => createGame({ name })} >
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
}
