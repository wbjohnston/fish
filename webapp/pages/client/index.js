import { Table } from 'antd'
import { useRouter } from 'next/router'
import { useEffect, useState } from 'react'
import Layout from '../../components/AuthedLayout'
import { listClients } from '../../lib/api'




export default function ListClients() {
    const router = useRouter();

    const [clients, setClients] = useState([])

    useEffect(() => {
        listClients().then(setClients)
    }, [])

    function onRow(row) {
        return {
            onClick: () => {
                router.push(`/client/${row.id}`)
            }
        }
    }

    const columns = [
        {
            title: 'Name',
            key: 'name',
            dataIndex: 'name',
        }
    ]

    return <Layout activeNavKey="clients">
        <h1>Clients</h1>
        <Table onRow={onRow} dataSource={clients} columns={columns} />
    </Layout>
}
