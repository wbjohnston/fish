import React from 'react';
import Head from 'next/head';
import useAuth from '../lib/hooks/UseAuth';
import LoginPage from './login';

export default function Home() {
  return <LoginPage />;
}
