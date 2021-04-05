import { Table } from 'antd'
import { useEffect, useState } from 'react'
import Layout from '../../components/Layout'
import { listClients } from '../../lib/api'




export default function ListClients() {

    const [clients, setClients] = useState([])

    useEffect(() => {
        listClients().then(x => {
            console.log(x)
            setClients(x)
        })
    }, [])


    const columns = [
        {
            title: 'Name',
            key: 'name',
            dataIndex: 'name',
        }
    ]

    return <Layout activeNavKey="clients">
        <Table dataSource={clients} columns={columns} />
    </Layout>
}
