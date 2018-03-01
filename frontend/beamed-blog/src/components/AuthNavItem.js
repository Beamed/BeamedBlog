import React, { Component } from 'react';
import {Navbar, Nav, NavItem} from 'react-bootstrap';

class AuthNavItem extends Component {
    constructor(props) {
        super(props);
        this.state = {};
        this.state.isAuthenticated = props.isAuthenticated;
    }
    
    render() {
        if(!this.state.isAuthenticated) {
            return (
            <NavItem href='/login'>
                Login
            </NavItem>
            )
        } else {
            return (
            <NavItem href='/logout'>
                Logout
            </NavItem>
            )
        }

    }
}

export default AuthNavItem;