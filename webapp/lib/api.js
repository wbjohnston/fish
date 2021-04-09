import axios from 'axios';

const client = axios.create({
  // TODO(will): source this from env
  url: 'http://localhost:8080',
  withCredentials: true,
});

export async function listUsers() {
  const response = await client.get('/user');

  return response.data;
}

export async function listGames() {
  const response = await client.get('/game');

  return response.data;
}

export async function createGame({ name }) {
  const response = await client.post('/game', { name });

  return response.data;
}

export async function fetchGame(id) {
  const response = await client.get(`/game/${id}`);

  return response.data;
}

export async function fetchUser(id) {
  const response = await client.get(`/user/${id}`);

  return response.data;
}

/**
 *
 * @param {string} username
 * @param {string} password
 * @returns Promise<string> token
 */
export async function login(username, password) {
  const response = await client.post('/auth/login', { username, password });

  return response;
}

export async function register(username, password) {
  const response = await client.post('/auth/register', { username, password });

  return response;
}

export async function logout() {
  const response = await axios.get('/auth/logout');

  return response.data;
}

export async function me() {
  const response = await axios.get('/me');
  return response.data;
}
