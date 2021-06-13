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

        // let url = 'http://localhost:8000/api/login/';
        let url = '/api/login/';
        axios.post(url, {
            "login": this.state.loginInput,
            "password": this.state.passwordInput
        }, {headers}).then(res => {
                let token = res.data.token;
                let rights = res.data.rights;

                cookies.set('token', token, {path: '/'});
                cookies.set('rights', rights, {path: '/'});

                this.props.history.push('/');
                window.location.reload();
            }
        ).catch((error) => {
            console.log('Error: ', error.message);
            let msg = error.message;

            if (msg.includes("401")) {
                msg = "Nieprawidłowy login lub hasło."
            }

            this.setState({errorMsg: msg});
        })
    }

    updateLoginInput(e) {
        this.setState({loginInput: e.target.value});
    }

    updatePasswordInput(e) {
        this.setState({passwordInput: e.target.value});
    }

    renderErrorCard() {
        if (this.state.errorMsg === "") {
            return;
        }

        return (
            <div className="alert alert-danger" role="alert">
                {this.state.errorMsg}
            </div>
        )
    }

    render() {
        return (
            <div>
                {this.renderErrorCard()}

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
