import React, {Component} from "react";
import axios from 'axios';
import Cookies from 'universal-cookie';

const cookies = new Cookies();

class Login extends Component {

    constructor(props) {
        super(props);

        this.state = {
            loginInput: "",
            passwordInput: "",
            errorMsg: ""
        };

        this.updateLoginInput = this.updateLoginInput.bind(this);
        this.updatePasswordInput = this.updatePasswordInput.bind(this);
        this.sendLogin = this.sendLogin.bind(this);
    }

    sendLogin() {
        const headers = {
            'Content-Type': 'application/json'
        };

        // let url = '/api/login/';
        let url = 'http://localhost:8000/api/login/';
        axios.post(url, {
            "login": this.state.loginInput,
            "password": this.state.passwordInput
        }, {headers}).then(res => {
                console.log(res);

                let token = res.data;
                console.log(cookies.get('token'));
                cookies.set('token', token, {path: '/'});
                console.log(cookies.get('token'));


                this.props.history.push('/');
                window.location.reload();
            }
        ).catch(function (error) {
            console.log('Error: ', error.message);
        })
    }

    updateLoginInput(e) {
        this.setState({loginInput: e.target.value});
    }

    updatePasswordInput(e) {
        this.setState({passwordInput: e.target.value});
    }

    render() {
        return (
            <div>
                <form>
                    <div className="mb-3 row">
                        <label htmlFor="staticEmail" className="col-sm-2 col-form-label">Login</label>
                        <div className="col-sm-10">
                            <input onChange={this.updateLoginInput} type="text" className="form-control"
                                   id="staticEmail" autoComplete="username"
                            />
                        </div>
                    </div>
                    <div className="mb-3 row">
                        <label htmlFor="inputPassword" className="col-sm-2 col-form-label">Hasło</label>
                        <div className="col-sm-10">
                            <input onChange={this.updatePasswordInput} type="password" className="form-control"
                                   id="inputPassword" autoComplete="current-password"/>
                        </div>
                    </div>
                    <button onClick={this.sendLogin} type="button" className="btn btn-primary btn-lg">Wyślij</button>
                </form>
            </div>
        );
    }
}

export default Login;
