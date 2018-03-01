import React, { Component } from 'react';
import '../styles/Login.css';

class Login extends Component {
    constructor(props) {
        super();
        this.state = {
            username: "",
            password: ""
        };
        this.handleChange = this.handleChange.bind(this);
        this.handleSubmit = this.handleSubmit.bind(this);
        this.authenticated = this.props.authenticated;    
    }

    handleSubmit(evt) {
        //alert('form submitted with: ' + this.state.username + ":" +this.state.password);
        evt.preventDefault();

    }
    handleChange(evt) {
        var partial_state = {};
        partial_state[evt.target.name] = evt.target.value;
        this.setState(partial_state);
    }

    render () {
        var handleSubmit = this.handleSubmit;
        var handleChange = this.handleChange;
        return (
            <div className="col-md-8 col-md-offset-2 content" id="login">
                <form onSubmit={handleSubmit}>
                    <div className="form-group">
                        <label htmlFor="username">Username</label>
                        <input type="text" value={this.state.username} onChange={handleChange} className="form-control" name="username" id="username" placeholder="Username" />
                    </div>
                    <div className="form-group">
                        <label htmlFor="password">Password</label>
                        <input type="password" className="form-control" value={this.state.password} onChange={handleChange} name="password" id="password" placeholder="Password" />
                    </div>
                    <button className="btn btn-primary" type="submit" >Submit</button>
                </form>
            </div>
        );
    }
}

export default Login;