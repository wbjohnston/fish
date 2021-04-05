import App from "next/app"
import "antd/dist/antd.css"
import useAuth, { ProvideAuth } from '../lib/hooks/UseAuth'



export default class MyApp extends App {
    render() {
        const { Component, pageProps } = this.props
        return <ProvideAuth>
            <Component {...pageProps} />
        </ProvideAuth>

    }
}
