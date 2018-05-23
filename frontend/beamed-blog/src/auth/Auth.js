const Auth = {
    isAuthenticated: false,
    onAuthentication : function(result) {
        if(result) {
            Auth.isAuthenticated = true;
        }
    },
};
export default Auth;