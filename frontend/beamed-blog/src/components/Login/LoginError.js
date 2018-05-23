import React, { Component } from 'react';
import {Alert} from 'react-bootstrap'

class LoginError extends Component {
    constructor(props) {
        super();
        this.props = props;
    }

    render() {
        if(this.props.authFailed) {
            return (
                <Alert bsStyle="danger">
                    <p>Your username and/or password are wrong. Please try again.</p>
                </Alert>
            )
        } 
        return null;
    }
}

export default LoginError;