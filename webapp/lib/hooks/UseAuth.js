import { useRouter } from 'next/router';
import { createContext, useContext, useEffect, useState } from 'react';
import { login, logout, me, register as fishRegister } from '../api';


const authContext = createContext();

export function ProvideAuth({ children }) {
    const auth = useProvideAuth();

    return <authContext.Provider value={auth}>{children}</authContext.Provider>
}

export function useProvideAuth() {
    const [user, setUser] = useState(null);


    function signin(username, password) {
        return login(username, password).then(user => {
            setUser(user)
            return user
        })
    }

    function register(username, password) {
        return fishRegister(username, password)
    }

    function signout() {
        return logout().then(_ => setUser(null))
    }

    useEffect(() => {
        me().then(setUser).catch(x => setUser(null))
    }, [])

    return {
        user,
        signin,
        register,
        signout
    }
}

export default function useAuth() {
    return useContext(authContext)
}
