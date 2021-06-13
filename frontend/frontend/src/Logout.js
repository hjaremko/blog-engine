import React, {Component} from "react";
import {Redirect} from 'react-router'
import Cookies from 'universal-cookie';

const cookies = new Cookies();

class Logout extends Component {
    render() {
        cookies.remove('token');
        this.props.history.push('/');
        window.location.reload();
        return <Redirect to='/'/>;
    }
}

export default Logout;
