import React, { useState, useEffect } from "react"

import { listUsers } from '../../lib/api'
import { useRouter } from 'next/router'
import Layout from "../../components/AuthedLayout"
import { Table } from "antd"

export default function ListUserPage() {
    const router = useRouter();
    const [users, setUsers] = useState([]);

    useEffect(() => {
        if (!router.isReady) {
            return
        }

        listUsers().then(setUsers)

    }, [])

    function onRow(row) {
        return {
            onClick: () => {
                router.push(`/user/${row.id}`)
            }
        }
    }

    const columns = [
        {
            title: "Username",
            key: 'username',
            dataIndex: 'username'
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
        <h2>Current Users</h2>
        <Table onRow={onRow} dataSource={users} columns={columns} />
    </Layout>
}
