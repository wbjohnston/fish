import App from "next/app"
import "antd/dist/antd.css"
import useAuth, { ProvideAuth } from '../lib/hooks/UseAuth'
import { ProvideWebsocket } from '../lib/hooks/UseWebsocket'



export default class MyApp extends App {
    render() {
        const { Component, pageProps } = this.props
        return <ProvideAuth>
            <ProvideWebsocket>
                <Component {...pageProps} />
            </ProvideWebsocket>
        </ProvideAuth>

    }
}
