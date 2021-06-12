import React, {Component} from "react";
import axios from 'axios';

class Blog extends Component {
    constructor(props) {
        super(props);

        this.state = {
            posts: [],
            error: '',
            loading: true
        };


        this.getBlogPosts = this.getBlogPosts.bind(this);
        this.renderBlogPosts = this.renderBlogPosts.bind(this);
    }

    renderSpinner() {
        if (!this.state.loading) {
            return;
        }

        return (
            <div className="d-flex justify-content-center">
                <div className="spinner-border" role="status">
                    <span className="visually-hidden">Loading...</span>
                </div>
            </div>
        )
    }

    getBlogPosts() {
        this.setState({loading: true});

        // let url = '/api/posts/';
        let url = 'http://localhost:8000/api/posts/';
        axios.get(url)
            .then(res => {
                console.log(res);

                let posts = res.data;
                this.setState({posts, loading: false});
            })
            .catch((e) => {
                console.log('Error: ', e.message);
                let error = e.message;
                this.setState({error: error, loading: false});
            })
    }

    renderPost(post, key) {
        function addHtmlNewlines(content) {
            let newContent = "";
            let objects = content.split("\n");

            for (let i = 0; i < objects.length; i++) {
                newContent += objects[i];
                newContent += "<br>";
            }

            return newContent;
        }

        return (
            <div className="blogpost" key={key}>
                <div className='post-header'>
                    <div className='titlebar'>
                        <span className='post-title'>{post.title}</span>
                        <span className='post-date'>{post.date} <i className="far fa-calendar-alt"></i></span>
                    </div>
                    <span className='post-author'><i className="fas fa-user-alt"></i> {post.author.name}</span>
                </div>
                <div className='post-content' dangerouslySetInnerHTML={{__html: addHtmlNewlines(post.content)}}/>
                <div className='post-footer'>
                    <div className='comment-link'>
                        Komentarze (0)
                    </div>
                    <div className='comment-link'>
                        Edytuj
                        Usuń
                    </div>
                </div>
            </div>
        )
    }

    renderBlogPosts() {
        if (this.state.loading) {
            return;
        }

        if (this.state.error !== '') {
            return (
                <div className="posts">
                    Error loading posts, sorry!
                </div>
            )
        }

        const list = this.state.posts.map((post) =>
            <div key={post.id}>
                {this.renderPost(post)}
            </div>
        );

        if (list.length === 0) {
            return ("Brak postów!")
        }

        return (
            <div className="posts">
                {list}
            </div>
        )
    }

    componentDidMount() {
        this.getBlogPosts()
    }

    render() {
        return (
            <div>
                {this.renderSpinner()}
                {this.renderBlogPosts()}
            </div>
        );
    }
}

export default Blog;