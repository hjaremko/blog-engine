import React, {Component} from "react";
import axios from 'axios';

class Blog extends Component {
    constructor(props) {
        super(props);

        this.state = {
            posts: [],
            error: '',
            loading: true,
            limit: 1,
            page: 0,
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
                <div className="spinner-border" role="status" ref={loadingRef => (this.loadingRef = loadingRef)}>
                    <span className="visually-hidden">Loading...</span>
                </div>
            </div>
        )
    }

    getBlogPosts(page, limit) {
        this.setState({loading: true});

        let url = `/api/posts/?page=${page}&limit=${limit}`;
        // let url = `http://localhost:8000/api/posts?page=${page}&limit=${limit}`;
        axios.get(url)
            .then(res => {
                let new_posts = res.data;

                if (new_posts.length === 0) {
                    console.log("No new posts!");
                    this.setState({loading: false});
                    return;
                }

                let posts = this.state.posts.concat(new_posts);
                console.log('Loaded ' + posts.length + ' posts');
                this.setState({posts});
            })
            .catch((e) => {
                console.log('Error: ', e.message);
                let error = e.message;
                this.setState({error: error});
                this.setState({loading: false});
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

    handleObserver(entities, observer) {
        if (this.state.loading) {
            this.setState({page: this.state.page + 1})
            this.getBlogPosts(this.state.page, this.state.limit)
        }
    }

    componentDidMount() {
        this.getBlogPosts(this.state.page, this.state.limit)

        var options = {
            root: null,
            rootMargin: "0px",
            threshold: 1.0
        };

        let observer = new IntersectionObserver(
            this.handleObserver.bind(this),
            options
        );

        observer.observe(this.loadingRef);
    }

    render() {
        return (
            <div>
                {this.renderBlogPosts()}
                {this.renderSpinner()}
            </div>
        );
    }
}

export default Blog;