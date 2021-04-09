import App from 'next/app';
import 'antd/dist/antd.css';
import React from 'react';
import { ProvideAuth } from '../lib/hooks/UseAuth';
import { ProvideWebsocket } from '../lib/hooks/UseWebsocket';

export default class MyApp extends App {
  render() {
    const { Component, pageProps } = this.props;
    return (
      <ProvideAuth>
        <ProvideWebsocket>
          {/* eslint-disable-next-line react/jsx-props-no-spreading */}
          <Component {...pageProps} />
        </ProvideWebsocket>
      </ProvideAuth>
    );
  }
}
