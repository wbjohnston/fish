import { Agent } from "http"
import axios from "axios"

/**
 * @returns Promise<object[]>
 */
export async function listClients() {
    const response = await axios.get('http://localhost:8080/client', {
        withCredentials: true,
    });

    return response.data
}

export async function listUsers() {
    const response = await axios.get("http://localhost:8080/user", { withCredentials: true })

    return response.data
}

export async function listGames() {
    const response = await axios.get("http://localhost:8080/game", { withCredentials: true })

    return response.data
}

export async function fetchClient(id) {
    const response = await axios.get(`http://localhost:8080/client/${id}`, { withCredentials: true })


    return response.data
}

export async function fetchGame(id) {
    const response = await axios.get(`http://localhost:8080/game/${id}`, { withCredentials: true })

    return response.data
}

export async function fetchUser(id) {
    const response = await axios.get(`http://localhost:8080/user/${id}`, { withCredentials: true })

    return response.data
}

/**
 * 
 * @param {string} username 
 * @param {string} password 
 * @returns Promise<string> token
 */
export async function login(username, password) {
    const response = await axios.post("http://localhost:8080/auth/login", { username, password }, {
        withCredentials: true
    });

    return response
}

export async function register(username, password) {
    const response = await axios.post("http://localhost:8080/auth/register",
        { username, password },
        { withCredentials: true }
    );



    return response
}

export async function logout() {
    const response = await axios.get("http://localhost:8080/auth/logout", { withCredentials: true })

    return response.data
}

export async function me() {
    const response = await axios.get("http://localhost:8080/me", { withCredentials: true })
    return response.data
}
