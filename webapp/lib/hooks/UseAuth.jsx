import React, {
  createContext, useContext, useEffect, useState,
} from 'react';

import {
  login, logout, me, register as fishRegister,
} from '../api';

const authContext = createContext();

export function useProvideAuth() {
  const [user, setUser] = useState(null);

  // TODO(will): add redirect logic here

  useEffect(() => {
    me().then(setUser).catch(() => setUser(null));
  }, []);

  function signin(username, password) {
    return login(username, password).then((user) => {
      setUser(user);
      return user;
    });
  }

  function register(username, password) {
    return fishRegister(username, password);
  }

  function signout() {
    return logout().then(() => setUser(null));
  }

  return {
    user,
    signin,
    register,
    signout,
  };
}

export function ProvideAuth({ children }) {
  const auth = useProvideAuth();

  return <authContext.Provider value={auth}>{children}</authContext.Provider>;
}

export default function useAuth() {
  return useContext(authContext);
}
