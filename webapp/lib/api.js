import { Agent } from "http"
import axios from "axios"


/**
 * @returns Promise<object[]>
 */
export async function listClients() {
    const response = await axios.get('http://localhost:8080/client', {}, {
        withCredentials: true,
    });

    console.log(response)

    return response.data
}

export async function listGames() {
    const response = await axios.get("http://localhost:8080/game", {}, { withCredentials: true })

    return response.data
}

export async function fetchClient(id) {
    const response = await axios.get(`http://localhost:8080/client/${id}`, {}, { withCredentials: true })


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

    console.log(response);


    return response
}

export async function register(username, password) {
    const response = await axios.post("http://localhost:8080/auth/register",
        { username, password },
        { withCredentials: true }
    );

    console.log(response)


    return response
}
