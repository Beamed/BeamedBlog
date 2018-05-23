import React, { Component } from 'react';
import {FormGroup, FormControl, ControlLabel, Button, Alert} from 'react-bootstrap'
import LoginError from '../components/Login/LoginError'
import Auth from "../auth/Auth"
import '../styles/Login.css';

class Login extends Component {
    constructor(props) {
        super();
        this.props = props;
        this.state = {
            username: "",
            password: "",
            authFailed : false,
        };
        this.handleChange = this.handleChange.bind(this);
        this.handleSubmit = this.handleSubmit.bind(this);
        this.onAuthentication = Auth.onAuthentication;
    }


    handleSubmit(evt) {
        var handleAuth = function(result) {
            this.onAuthentication(result.ok);
            if(!result.ok) {
                handleError();
            }
        }.bind(this);
        var handleError = function() {
            this.setState({authFailed: true});
        }.bind(this);
        fetch("/api/login", {
            method: "POST",
            headers: {"Content-Type": "application/json",},
            body : JSON.stringify({
                username: this.state.username,
                password: this.state.password,
            })
        }).then( handleAuth, 
            handleError
        );
        evt.preventDefault();

    }
    handleChange(evt) {
        var partial_state = {};
        partial_state[evt.target.name] = evt.target.value;
        this.setState(partial_state);
    }

    render () {
        return (
            <div className="col-md-8 col-md-offset-2 content" id="login">
                <LoginError authFailed={this.state.authFailed}/>
                <form onSubmit={this.handleSubmit}>
                    <FormGroup controlId="username">
                        <ControlLabel>Username</ControlLabel>
                        <FormControl type="text" value={this.state.username} onChange={this.handleChange} name="username" placeholder="Username" />
                    </FormGroup>
                    <FormGroup controlId="password">
                        <ControlLabel>Password</ControlLabel>
                        <FormControl type="password" value={this.state.password} onChange={this.handleChange} name="password" placeholder="Password" />
                    </FormGroup>
                    <Button type="submit" bsStyle="primary">Submit</Button>
                </form>
            </div>
        );
    }
}

export default Login;