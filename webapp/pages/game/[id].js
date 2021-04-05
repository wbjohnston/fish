import 'antd/dist/antd.css'
import { Table } from 'antd'

export default function ListGamePage() {
    const colums = [
        {
            title: "name",
            dataIndex: 'name'
        },
        {
        }
    ]

    const dummy = [
        {
            name: "foobar"
        }
    ];


    return <Table dataSource={data} columns={columns} />
}
