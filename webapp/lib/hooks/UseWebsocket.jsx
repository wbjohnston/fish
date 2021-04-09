import useAuth from './UseAuth';

const {
  createContext, useContext, useState, useEffect,
} = require('react');

const websocketContext = createContext();

export function ProvideWebsocket({ children }) {
  const websocket = useProvideWebsocket();

  return <websocketContext.Provider value={websocket}>{children}</websocketContext.Provider>;
}

export function useProvideWebsocket() {
  const { user } = useAuth();
  const [websocket, setWebsocket] = useState(null);

  useEffect(() => {
    if (!user) {
      setWebsocket(null);
    }

    setWebsocket(new WebSocket('ws://localhost:8080/me/ws'));
  }, [user]);

  return {
    websocket,
  };
}

export default function useWebsocket() {
  return useContext(websocketContext);
}
