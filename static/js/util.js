export async function setUserSessionInfo(user_id) {
    const user_info = await fetch(`/api/users/${user_id}`).then((response) =>
        response.json()
    );
    user_info.toString = function () {
        return JSON.stringify(this);
    };
    // sessionStorage = user_info;
    Object.assign(sessionStorage, user_info);
    console.log(`User info: ${JSON.stringify(sessionStorage)}`);
}
