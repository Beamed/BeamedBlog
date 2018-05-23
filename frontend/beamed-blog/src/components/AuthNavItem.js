import React, { Component } from 'react';
import {Navbar, Nav, NavItem} from 'react-bootstrap';
import Auth from '../auth/Auth';

class AuthNavItem extends Component {
    constructor(props) {
        super(props);
        this.state = {};
    }
    
    render() {
        if(!Auth.isAuthenticated) {
            return (
            <NavItem href='/login' onAuthentication={Auth.onAuthentication}>
                Login
            </NavItem>
            )
        } else {
            return (
            <NavItem href='/logout' onAuthentication={Auth.onAuthentication}>
                Logout
            </NavItem>
            )
        }

    }
}

export default AuthNavItem;